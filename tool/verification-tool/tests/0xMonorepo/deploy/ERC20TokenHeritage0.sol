// SPDX-License-Identifier: MIT
pragma solidity >= 0.5.0;

contract ERC20TokenHeritage0 {

    mapping (address => uint) balances;
    mapping (address => mapping (address => uint)) allowed;
    
    function balanceOf(address _owner)
        public
        view
        returns (uint balance)
    {
        return balances[_owner];
    }

    function allowance(address _owner, address _spender) 
        public
        view
        returns (uint remaining)
    {
        return allowed[_owner][_spender];
    }

    
}