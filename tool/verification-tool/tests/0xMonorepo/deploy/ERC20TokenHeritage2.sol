// SPDX-License-Identifier: MIT
pragma solidity >= 0.5.0;
import "./ERC20TokenHeritage1.sol";

contract ERC20TokenHeritage2 is ERC20TokenHeritage1 {

    
    function approve(address _spender, uint _value) 
        public
        returns (bool success)
    {
        allowed[msg.sender][_spender] = _value;
        emit Approval(msg.sender, _spender, _value);
        return true;
    }
}