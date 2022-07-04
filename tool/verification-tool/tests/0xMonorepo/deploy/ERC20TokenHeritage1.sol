// SPDX-License-Identifier: MIT
pragma solidity >= 0.5.0;
import "./ERC20TokenHeritage0.sol";

contract ERC20TokenHeritage1 is ERC20TokenHeritage0 {

    uint public _totalSupply;

    event Transfer(address indexed _from, address indexed _to, uint _value);
    event Approval(address indexed _owner, address indexed _spender, uint _value);

    function transfer(address _to, uint _value)
        public
        returns (bool success) 
    {
        require(balances[msg.sender] >= _value && balances[_to] + _value >= balances[_to]); 
        balances[msg.sender] -= _value;
        balances[_to] += _value;
        // emit Transfer(msg.sender, _to, _value);
        return true;
    }

    function transferFrom(address _from, address _to, uint _value)
        public
        returns (bool success) 
    {
        require(balances[_from] >= _value && allowed[_from][msg.sender] >= _value && balances[_to] + _value >= balances[_to]); 
        balances[_to] += _value;
        balances[_from] -= _value;
        allowed[_from][msg.sender] -= _value;
        // emit Transfer(_from, _to, _value);
        return true;
    }
}