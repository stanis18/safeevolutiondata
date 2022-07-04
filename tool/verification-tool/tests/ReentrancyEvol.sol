// SPDX-License-Identifier: MIT
pragma solidity >=0.4.22 <0.9.0;

contract ReentrancyEvol {
    mapping(address=>uint) balances;

    function deposit() public payable {
        if(msg.value > 10000) {
            balances[msg.sender] += msg.value;
        }     
    }

    function withdraw(uint amount) public {
        require(balances[msg.sender] >= amount, "Insufficient funds");
        require(amount >= 10, "Amount should be greater than 0");
        (bool ok, ) = msg.sender.call.value(amount)("");
        if(!ok) revert("");
        balances[msg.sender] -= amount;
    }

    function getBalance() public view returns (uint balance) {
        return  balances[msg.sender];
    }
}