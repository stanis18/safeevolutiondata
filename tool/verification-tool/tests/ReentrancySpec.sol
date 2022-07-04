// SPDX-License-Identifier: MIT
pragma solidity >=0.4.22 <0.9.0;

contract ReentrancySpec {
    mapping(address=>uint) balances;

    function deposit() public payable {
    }
    
    function withdraw(uint amount) public {
    }

    /// @notice postcondition balances[msg.sender] == balance
    function getBalance() public view returns (uint balance) {
    }
}