
import Web3 from "web3";

export async function getConfirmations(txHash) {
    try {
      // Instantiate web3 with HttpProvider
      let web3 = new Web3(window.ethereum);
      // Get transaction details
      const trx = await web3.eth.getTransaction(txHash);
      // Get current block number
      const currentBlock = await web3.eth.getBlockNumber();
      // When transaction is unconfirmed, its block number is null.
      // In this case we return 0 as number of confirmations
      return trx.blockNumber === null ? 0 : currentBlock - trx.blockNumber;
    }
    catch (error) {
      console.log(`getConfirmations()  - ${error.message}`);
    }
  }

  function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
  }

 export async function confirmEtherTransaction(txHash, confirmations = 1) {

    let confirmed = false;

    do {
          
      const trxConfirmations = await getConfirmations(txHash);
      console.log('Transaction with hash ' + txHash + ' has ' + trxConfirmations + ' confirmation(s)');

      if (trxConfirmations >= confirmations){
        // Handle confirmation event according to your business logic
        console.log('Transaction with hash ' + txHash + ' has been successfully confirmed ' + trxConfirmations + ' confirmations');
        confirmed = true;
      } else {  
        await sleep(3 * 1000);
      }       
    } while( !confirmed );
  }