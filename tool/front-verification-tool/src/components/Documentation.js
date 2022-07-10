import React from 'react';
import backfrase from '../documentation/backfrase.png';
import importaccount from '../documentation/importaccount.jpeg';
import importaccount3 from '../documentation/importaccount3.jpeg';
import changenetwork from '../documentation/changenetwork.png';
import addnetwork from '../documentation/addnetwork.png';
import selectnetwork from '../documentation/selectnetwork.png';
import addnetwork3 from '../documentation/addnetwork3.png';

const Documentation = () => {

    return (
        <div className="grid">
            <div className="col-12">
                <div className="card docs">
                <h3>Getting Started</h3>


                <h4>What is MetaMask?</h4>
                <p>Developed by ConsenSys in 2016, MetaMask is the most used Web3 browser wallet. 
                    Originally designed to interact with the leading 
                    smart contract chain, Ethereum, MetaMask now caters to a range of blockchains that are 
                    compatible with EVM (Ethereum Virtual Machine). Moreover, users can seamlessly transition 
                    between networks facilitating a single end-to-end experience. However, MetaMask remains to 
                    serve as an Ethereum wallet by default. </p>    
                
                <h4>How to Set Up a MetaMask Wallet on a Desktop</h4>

                <p> Before the first step of our MetaMask tutorial, head to the official <a href="https://metamask.io/">MetaMask site</a> to find the download 
                    button. There should be two buttons available as soon as you land on the site. 
                    The first is a “Download Now” button in the center of the homepage, with a second static 
                    “Download” button in the top-right corner. Now, click on one of those to move on to the 
                    first step!</p>

                    <ol>
                    <li> <b>Choose a Browser:</b> After clicking on “Download”, MetaMask will redirect you to the download page to confirm 
                        the relevant browser. Moreover, you can download MetaMask on multiple web browsers. 
                        This includes Chrome, Firefox, Brave, and Edge. However, you can only install MetaMask 
                        on the browser you’re using at the time. 
                       
                        Select the applicable browser you’re using. This will automatically send you to a browser 
                        confirmation page prompting action to confirm the installation. Confirm that you’d like to 
                        add the extension – there may be multiple confirmation steps depending on what browser 
                        you opt for. </li>

                        <br></br><br></br>
                    
                    
                        <li> <b>Creating a new account:</b> After installing the extension, the second step is to choose 
                        between setting up a new wallet or importing an existing one using a seed phrase. 
                        By selecting “Create a new wallet”, MetaMask will ask you to set a password for your wallet. 
                        Now, MetaMask will ask you to do the important task regarding the safety of your funds 
                        – creating a backup phrase. 
                        
                        <br></br>
                        <img src={backfrase} width="500" height="500"></img>
                        <br></br>
                        
                        At this stage, users will need a pen and paper to write down the twelve words revealed in the order they appear. 
                        This seed phrase is a recovery backup phrase, allowing users to gain access to their wallets from any device. 
                        However, if this phrase is lost, there is no alternative way of retrieving funds from that account.
                        </li>

                        <br></br><br></br>
                    
                        <li> <b>Import an account :</b> If you have a seed words (mnemonic phrase) you can import your wallet by clicking on 
                                the icon in the upper right corner to open MetaMask extension, read and accept the terms.
                                Click Import With Seed Phrase. Enter your wallets seed phrase (mnemonic phrase).
                                 Enter a strong password and click Import. </li>

                        <br></br>
                        <img src={importaccount} width="250" height="300"></img> <img src={importaccount3} width="250" height="300"></img>
                        <br></br>
                    </ol>  
                        
                    <h4>Change the MetaMask network</h4>

                    When you open your MetaMask wallet, you will be able to see your current network at the top of the display. 
                    You can click on the down arrow next to where your current network is mentioned. 
                    After dropping down, you will see a list of different available networks. 
                    
                    <br></br>
                    <br></br>
                        <img src={changenetwork} width="250" height="300"></img>
                    <br></br>
                    <br></br>


                    All you have to do is to select a network out of the options that you can see in here and proceed with changing the network. 
                    The MetaMask network change will happen immediately.
                    <br></br>
                    <br></br>
                        <img src={addnetwork} width="250" height="300"></img>
                    <br></br>

                    Next, click on your profile picture, which you can see on the top right hand corner. 
                    Doing so will allow you to navigate to Settings in the pop-up menu. 
                    When you’ve clicked, select the option saying Networks.

                    <br></br>
                    <br></br>
                        <img src={selectnetwork} width="250" height="300"></img>
                    <br></br>
                    <br></br>

                    Once you select Networks, you’ll see all the networks that you have added. 
                    On the bottom of that menu, you can see a new button named “Add Network”. 
                    You may click on it and proceed with adding a new network. 
                    Once you are done with adding the network, you may click on the Save button.

                    <br></br>
                    <br></br>
                        <img src={addnetwork3} width="250" height="300"></img>
                    <br></br>

                    <h4>Networks to add MetaMask</h4>

                    Once the networks are added you can add credits through the respective faucets.

                    <br></br>

                    <h5>Celo (Alfajores Testnet)</h5>

                    <b>RPC URL:</b> https://alfajores-forno.celo-testnet.org <br></br>
                    <b>Chain ID:</b> 44787 <br></br>
                    <b>Symbol:</b> CELO <br></br>
                    <b>Block explorer:</b> https://alfajores-blockscout.celo-testnet.org <br></br>
                    <b>Faucet:</b> https://celo.org/developers/faucet


                    <h5>Energyweb (Volta)</h5> 
                    <b>RPC URL:</b> https://volta-rpc.energyweb.org <br></br>
                    <b>Chain ID:</b> 73799  <br></br>
                    <b>Symbol:</b> VT <br></br>
                    <b>Block explorer:</b> https://volta-explorer.energyweb.org/ <br></br>
                    <b>Faucet:</b> https://voltafaucet.energyweb.org/ <br></br>

                    <br></br>

                    You can also deploy smart contracts to a local network with ganache which allows us to develop and test smart contract easily.
                    Ganache can be downloaded and installed from the <a href="https://trufflesuite.com/ganache/">official site from Truffle suite.</a>
                    The accounts created are created as part of a HD wallet with the mnemonic given in the UI. 
                    Using this mnemonic we can import the accounts in a wallet like Metamask and connect to our local blockhain.
                    Once added, we can see that our account on the Ganache Local network has 100 ETH as expected since 
                    we are running on the blockchain setup by Ganache client. After deploying, you can make any transfer, for example we can import 
                    a ganache wallet can transfer some Ether to your private account.
                  
                    
                    <br></br>

                    <h5>Ganache</h5>

                    <b>RPC URL:</b> http://127.0.0.1:7545 <br></br>
                    <b>Chain ID:</b> 1337 <br></br>
                    <b>Symbol:</b> ETH <br></br>



                    <h4>References</h4>

                    <ol>
                    <li> https://metaschool.so/articles/how-to-change-add-new-network-metamask-wallet/</li>
                    <li> https://medium.com/publicaio/how-import-a-wallet-to-your-metamask-account-dcaba25e558d</li>
                    <li> https://academy.moralis.io/blog/metamask-tutorial-how-to-set-up-a-metamask-wallet</li>
                    <li> https://kimsereylam.com/metamask/2022/02/25/setup-local-development-blockchain-with-ganache.html</li>
                    </ol>

                </div>
            </div>
        </div>
    )
}

const comparisonFn = function (prevProps, nextProps) {
    return prevProps.location.pathname === nextProps.location.pathname;
};

export default React.memo(Documentation, comparisonFn);
