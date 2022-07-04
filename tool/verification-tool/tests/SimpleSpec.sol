// SPDX-License-Identifier: MIT
pragma solidity >=0.4.22 <0.9.0;

/// @notice  invariant a == a
contract SimpleSpec {

    uint a;
    uint b;
    bool c;

    /// @notice postcondition a == _a
    /// @notice postcondition b == _b
    /// @notice postcondition c == _c
    constructor(uint _a, uint _b, bool _c) public {
    }

   
    /// @notice  postcondition c == _c
    function set_c(bool _c) public {
    }
    

    /// @notice postcondition a == _a
    function set_a(uint _a) public {
    }

    /// @notice postcondition resp == a || resp == b
    function get_selected() view public returns (uint resp) {
    }

    /// @notice postcondition b == _b
    function set_b(uint _b) public {
    }
    
}