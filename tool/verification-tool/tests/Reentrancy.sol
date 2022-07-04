// SPDX-License-Identifier: MIT
pragma solidity >=0.4.22 <0.9.0;

contract Reentrancy {
    mapping(address=>uint) balances;

    function deposit() public payable {
        balances[msg.sender] += msg.value;
    }

    function withdraw(uint amount) public {
        require(balances[msg.sender] >= amount, "Insufficient funds");
        (bool ok, ) = msg.sender.call.value(amount)("");
        if(!ok) revert("");
        balances[msg.sender] -= amount;
    }

    function getBalance() public view returns (uint balance) {
        return  balances[msg.sender];
    }
}