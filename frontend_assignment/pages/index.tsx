import detectEthereumProvider from "@metamask/detect-provider"
import { Box, Button, TextField } from "@mui/material"
import { Strategy, ZkIdentity } from "@zk-kit/identity"
import { generateMerkleProof, Semaphore } from "@zk-kit/protocols"
import { Contract, providers, utils } from "ethers"
import Head from "next/head"
import React, { useState } from "react"
import styles from "../styles/Home.module.css"
import { useForm, Controller } from 'react-hook-form';
import { yupResolver } from '@hookform/resolvers/yup';
import * as yup from "yup";
import Greeter from "artifacts/contracts/Greeters.sol/Greeters.json"

  const contract = new Contract("0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512", Greeter.abi)
  const provider = new providers.JsonRpcProvider("http://localhost:8545")
  const contractOwner = contract.connect(provider.getSigner())

export default function Home() {
    const [logs, setLogs] = React.useState("Connect your wallet and greet!")
   
    const schema = yup.object({
        name: yup.string().required(),
        age: yup.number().positive().integer().required(),
        address: yup.string().required()
      }).required();
    const { control, handleSubmit, formState:{ errors } } = useForm({
        resolver: yupResolver(schema)
      });
    const onSubmit = (data: any) => {
        console.log(data);
    }
    const greeting = "Hello world"
    const [greeti, setGreet] = useState("")

    contractOwner.on("NewGreeting", (result) => {
        let greetStr = utils.parseBytes32String(result)
        setGreet(greetStr)
        console.log(greet)
    })

    async function greet() {
        setLogs("Creating your Semaphore identity...")

        const provider = (await detectEthereumProvider()) as any

        await provider.request({ method: "eth_requestAccounts" })

        const ethersProvider = new providers.Web3Provider(provider)
        const signer = ethersProvider.getSigner()
        const message = await signer.signMessage("Sign this message to create your identity!")

        const identity = new ZkIdentity(Strategy.MESSAGE, message)
        const identityCommitment = identity.genIdentityCommitment()
        const identityCommitments = await (await fetch("./identityCommitments.json")).json()

        const merkleProof = generateMerkleProof(20, BigInt(0), identityCommitments, identityCommitment)

        setLogs("Creating your Semaphore proof...")


        const witness = Semaphore.genWitness(
            identity.getTrapdoor(),
            identity.getNullifier(),
            merkleProof,
            merkleProof.root,
            greeting
        )

        const { proof, publicSignals } = await Semaphore.genProof(witness, "./semaphore.wasm", "./semaphore_final.zkey")
        const solidityProof = Semaphore.packToSolidityProof(proof)

        const response = await fetch("/api/greet", {
            method: "POST",
            body: JSON.stringify({
                greeting,
                nullifierHash: publicSignals.nullifierHash,
                solidityProof: solidityProof
            })
        })

        if (response.status === 500) {
            const errorMessage = await response.text()

            setLogs(errorMessage)
        } else {
            setLogs("Your anonymous greeting is onchain :)")
        }
    }   

    return (
        <div className={styles.container}>
            <Head>
                <title>Greetings</title>
                <meta name="description" content="A simple Next.js/Hardhat privacy application with Semaphore." />
                <link rel="icon" href="/favicon.ico" />
            </Head>

            <main className={styles.main}>
                <h1 className={styles.title}>Greetings</h1>

                <p className={styles.description}>A simple Next.js/Hardhat privacy application with Semaphore.</p>

                <div className={styles.logs}>{logs}</div>

                <Box component="form" onSubmit={handleSubmit(onSubmit)} noValidate sx={{ mt: 1 }}>
                    <Controller
                        name="name"
                        control={control}
                        render={({ field: { onChange, value } }) => (
                            <TextField 
                                margin="normal"
                                fullWidth
                                required 
                                onChange={onChange} 
                                value={value || ''}
                                label="Name" />
                          )}
                    />
                    <p>{errors.name?.message}</p>
                    <Controller
                        name="age"
                        control={control}
                        render={({ field: { onChange, value } }) => (
                            <TextField
                                margin="normal"
                                required
                                fullWidth
                                label="Age"
                                value={value || ''}
                                onChange={onChange} 
                                type="number"
                            />
                        )}
                    />
                    <p>{errors.age?.message}</p>
                    <Controller
                        name="address"
                        control={control}
                        render={({ field: { onChange, value } }) => (
                            <TextField
                                margin="normal"
                                required
                                fullWidth
                                label="Address"
                                value={value || ''}
                                onChange={onChange} 
                                placeholder="100 Smallville, Kansas"
                            />
                        )}
                    />
                    <p>{errors.address?.message}</p>
                    <Button
                        onClick={handleSubmit(onSubmit)}
                        fullWidth
                        variant="contained"
                        sx={{ mt: 3, mb: 2 }}
                    >       
                        Submit
                    </Button>
                </Box>

                <div onClick={() => greet()} className={styles.button}>
                    Greet
                </div>
                <div>
                    <p>{greeti}</p>
                </div>
            </main>
        </div>
    )
}
