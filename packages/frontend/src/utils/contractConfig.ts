// src/utils/contractConfig.ts

import { ethers } from 'ethers';
import contractABI from '../contracts/Counter.json';
import deploymentInfo from '../contracts/deployment.json';

export const getContractConfig = () => {
    return {
        address: process.env.NEXT_PUBLIC_CONTRACT_ADDRESS || deploymentInfo.address,
        abi: contractABI,
        network: deploymentInfo.network
    };
};

export const getContract = async () => {
    if (typeof window === 'undefined') {
        return null;
    }

    const { address, abi } = getContractConfig();

    if (!window.ethereum) {
        throw new Error('Please install MetaMask');
    }

    const provider = new ethers.providers.Web3Provider(window.ethereum);
    const signer = provider.getSigner();

    return new ethers.Contract(address, abi, signer);
};