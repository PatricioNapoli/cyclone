
<p align="center">
    <img alt="Cyclone" src="assets/cyclone.jpg" width="400px"/>
</p>

<div align="center">

  <a style="margin-right:15px" href="#"><img src="https://forthebadge.com/images/badges/made-with-rust.svg" alt="Made with Rust"/></a>


  <a href="https://opensource.org/licenses/MIT"><img src="https://img.shields.io/badge/License-MIT-brightgreen.svg" alt="License MIT"/></a>
  <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/rust-1.64-orange.svg" alt="Rust 1.64"/></a>

</div>


# Cyclone

Project heavily in progress.

## Overview

Rust based distributed database with coordinator-shard-slave modes, with TCP or unix socket support and Dashmap usage.

## Prerequisites

rust 1.64

## Build & Run

`cargo run`  

It will listen on cyclone.sock file.  

Connect through bash with:  

`nc -U cyclone.sock`  

Then you may send either:

`set KEY VALUE`  

or

`get KEY`  
