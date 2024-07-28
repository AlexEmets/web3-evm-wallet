// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract Counter {
    uint256 private count;

    // Встановлює початкове значення лічильника
    constructor(uint256 initialCount) {
        count = initialCount;
    }

    // Інкрементує значення лічильника на 1
    function increment() public {
        count += 1;
    }

    // Отримує поточне значення лічильника
    function getCount() public view returns (uint256) {
        return count;
    }
}
