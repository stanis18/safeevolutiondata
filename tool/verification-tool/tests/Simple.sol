// SPDX-License-Identifier: MIT
pragma solidity >=0.4.22 <0.9.0;

contract Simple {

    uint a;
    uint b;
    bool c;

    function get_selected() view public returns (uint resp) {
        if(c) {
            return a;
        } else {
            return b;
        }
    }

    constructor(uint _a, uint _b, bool _c) public {
    	a = _a;
		b = _b;
		c = _c;
    }

    function set_a(uint _a) public {
        a = _a;
    }

    function set_b(uint _b) public {
        b = _b;
    }

    function set_c(bool _c) public {
        c = _c;
    }	
}