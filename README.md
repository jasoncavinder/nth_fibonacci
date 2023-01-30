<a name="readme-top"></a>

[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![Unlicense][license-shield]][license-url]
[![LinkedIn][linkedin-shield]][linkedin-url]

<!-- PROJECT LOGO -->
<br />
<div align="center">

<h3 align="center">nth Fibonacci</h3>

  <p align="center">
    Generate the nth Fibonacci number.
    <br />
    <a href="https://github.com/jasoncavinder/nth_fibonacci"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/jasoncavinder/nth_fibonacci/issues">Report Bug</a>
    ·
    <a href="https://github.com/jasoncavinder/nth_fibonacci/issues">Request Feature</a>
  </p>
</div>

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
    <li><a href="#contact">Contact</a></li>
  </ol>
</details>

<!-- ABOUT THE PROJECT -->

## About The Project

[Convert Temp Screen Shot][product-screenshot]

I'm learning Rust by working through ["The Book"](https://doc.rust-lang.org/stable/book/) and building simple projects to practice the concepts discussed in various chapters. This is one of those projects.

### This project was based on the following prompt:

You made it! This was a sizable chapter: you learned about variables, scalar and compound data types, functions, comments, if expressions, and loops! To practice with the concepts discussed in this chapter, try building programs to do the following:

- [ ] Convert temperatures between Fahrenheit and Celsius.
- [x] Generate the nth Fibonacci number.
- [ ] Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

### Built With

- [![Rust][rust-lang]][rust-url]
- [![VS Code][vs-code]][vs-code-url]

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- GETTING STARTED -->

## Getting Started

To get a local copy up and running follow these simple example steps.

### Prerequisites

Rust

- Linux/macOS
  ```sh
  curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
  ```
- Windows  
  [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

\[Optional\] Install a C compiler

- Linux
  ```sh
  sudo apt install build-essential
  ```
- macOS
  ```sh
  xcode-select --install
  ```
- Windows  
  [https://visualstudio.microsoft.com/downloads/](https://visualstudio.microsoft.com/downloads/)

### Installation

1. Clone the repo
   ```sh
   git clone https://github.com/jasoncavinder/nth_fibonacci.git
   ```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- USAGE EXAMPLES -->

## Usage

Build & Run

```sh
cargo run
```

```
./nth_fibonacci;\
./nth_fibonacci 1;\
./nth_fibonacci 2;\
./nth_fibonacci 3;\
./nth_fibonacci 4;\
./nth_fibonacci 5;\
./nth_fibonacci 6;\
./nth_fibonacci 7;\
./nth_fibonacci 8;\
./nth_fibonacci 19;\
./nth_fibonacci 42;\
./nth_fibonacci 99;\
./nth_fibonacci asdf;\
sleep 60
Usage: ./nth_fibonacci <number>
The 1st fibonacci number is 1.
The 2nd fibonacci number is 1.
The 3rd fibonacci number is 2.
The 4th fibonacci number is 3.
The 5th fibonacci number is 5.
The 6th fibonacci number is 8.
The 7th fibonacci number is 13.
The 8th fibonacci number is 21.
The 19th fibonacci number is 4,181.
The 42nd fibonacci number is 267,914,296.
The number you entered is too large for this program to handle.
FYI, the 93rd fibonacci number is 12,200,160,415,121,876,738.
The 0th fibonacci number is 0.
```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- ROADMAP -->

## Roadmap

- [ ] Add comments and documentation
- [ ] Optimize code

See the [open issues](https://github.com/othneildrew/Best-README-Template/issues) for a full list of proposed features (and known issues).

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- LICENSE -->

## License

Distributed under the Unlicense. See `UNLICENSE.md` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- ACKNOWLEDGEMENTS -->

## Acknowledgements

(book) [The Rust Programming Language](https://doc.rust-lang.org/stable/book/)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- CONTACT -->

## Contact

Jason Cavinder - [@jasoncavinder](https://twitter.com/jasoncavinder) - jason.cavinder@gmail.com - [Jason Cavinder][linkedin-url]

Project Link: [https://github.com/jasoncavinder/nth_fibonacci](https://github.com/jasoncavinder/nth_fibonacci)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- MARKDOWN LINKS & IMAGES -->

[contributors-shield]: https://img.shields.io/github/contributors/jasoncavinder/nth_fibonacci.svg?style=for-the-badge
[contributors-url]: https://github.com/jasoncavinder/nth_fibonacci/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/jasoncavinder/nth_fibonacci.svg?style=for-the-badge
[forks-url]: https://github.com/jasoncavinder/nth_fibonacci/network/members
[stars-shield]: https://img.shields.io/github/stars/jasoncavinder/nth_fibonacci.svg?style=for-the-badge
[stars-url]: https://github.com/jasoncavinder/nth_fibonacci/stargazers
[issues-shield]: https://img.shields.io/github/issues/jasoncavinder/nth_fibonacci.svg?style=for-the-badge
[issues-url]: https://github.com/jasoncavinder/nth_fibonacci/issues
[license-shield]: https://img.shields.io/github/license/jasoncavinder/nth_fibonacci.svg?style=for-the-badge
[license-url]: https://github.com/jasoncavinder/nth_fibonacci/blob/master/UNLICENSE.txt
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://linkedin.com/in/jason-cavinder
[product-screenshot]: images/screenshot.png
[rust-lang]: https://img.shields.io/badge/Rust-120712?style=for-the-badge&logo=Rust&logoColor=B94700
[rust-url]: https://www.rust-lang.org/
[vs-code]: https://img.shields.io/badge/VS_Code-000?style=for-the-badge&logo=visualstudiocode&logoColor=0078D7
[vs-code-url]: https://code.visualstudio.com/

<!--
README.md based on [othneildrew/Best-README-Template]: https://github.com/othneildrew/Best-README-Template

MIT License

Copyright (c) 2021 Othneil Drew

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE. -->
