<a name="readme-top"></a>


[![MIT License][license-shield]][license-url]
[![LinkedIn][linkedin-shield]][linkedin-url]


<!-- PROJECT LOGO -->
<br />
  <div align="center">

```

 __    __                      __                           
|  \  |  \                    |  \                          
| $$  | $$  ______    _______ | $$____    ______    ______  
| $$__| $$ |      \  /       \| $$    \  /      \  /      \ 
| $$    $$  \$$$$$$\|  $$$$$$$| $$$$$$$\|  $$$$$$\|  $$$$$$\
| $$$$$$$$ /      $$ \$$    \ | $$  | $$| $$    $$| $$   \$$
| $$  | $$|  $$$$$$$ _\$$$$$$\| $$  | $$| $$$$$$$$| $$      
| $$  | $$ \$$    $$|       $$| $$  | $$ \$$     \| $$      
 \$$   \$$  \$$$$$$$ \$$$$$$$  \$$   \$$  \$$$$$$$ \$$      
                                                            


```
  <i><p align="center"> ASCII art was created with the help of <a href="https://patorjk.com/software/taag/">patorjk.com</a></p></i>
  
  <br>
  <h3 align="center">Hasher</h3>

  <p align="center">
    A command line utility for calculating cryptographic hashes. 
    <br />
    <br />
    <a href="https://github.com/PeanutButterRat/hasher/issues">Report Bug</a>
    ·
    <a href="https://github.com/PeanutButterRat/hasher/issues">Request Feature</a>
  </p>
</div>



<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li>
      <a href="#usage">Usage</a>
      <ul>
        <li><a href="#options">Options</a></li>
        <li><a href="#algorithms">Algorithms</a></li>
      </ul>
    </li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

Hasher is my attempt at implmenting some popular cryptographic hashing functions such as those from SHA-2. It was developed from the offical specifications as a bit of a challenge.

Hasher is not meant to be a replacement for any real hashing tools. It is solely a learning experience.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- GETTING STARTED -->
## Getting Started

To compile, follow these simple steps shown below. Alternatively, you can simply download the binary from the GitHub page under [releases](https://github.com/PeanutButterRat/hasher/releases/).

### Prerequisites

If you wish to compile Hasher from source, the only thing you will need is a Rust installation. You can download the latest version of the Rust compiler from [https://www.rust-lang.org/](https://www.rust-lang.org/tools/install).

### Installation

1. Clone the repo
   ```sh
   git clone https://github.com/PeanutButterRat/hasher.git
   ```
2. Compile Hasher
   ```sh
   cd hasher  # You should be in the repo directory now.
   cargo build
   ```

Hasher should now be ready to use in the ```hasher/build``` folder. See #Usage for more details. 


<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- USAGE EXAMPLES -->
## Usage

```
hasher <ALGORITHM> [OPTIONS] <FILE>
```

Example:
```
hasher sha256 -i abc  # Hashes the string "abc" with SHA-256.
```

<!-- SUPPORTED OPTIONS -->
### Options
Shown below is a list of all the command line options that Hasher currently supports. This list may be updated in the future if some features are added.

| Option      | Short   | Long          | Description                                                 |
|:-----------:|:-------:|:-------------:|:-----------------------------------------------------------:|
| Immediate   | -i      | --immediate   | Hash the FILE argument as input instead of opening a file   |

<!-- SUPPORTED ALGORITHMS -->
### Algorithms

Shown below is a list of all the algorithms that Hasher currently supports. This list may be updated in the future as more algorithms are implemented.

| Algorithm   | Command Line Argument   |
|:-----------:|:-----------------------:|
| SHA-256     | -sha256                 |


<!-- CONTRIBUTING -->
## Contributing

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with and present your idea there. If for some reason you feel inspired enough to contribute, any contribution would be greatly appreciated!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTACT -->
## Contact

Eric Brown - [GitHub](https://github.com/PeanutButterRat) - ebrownbus@gmail.com

Project Link: [https://github.com/PeanutButterRat/hasher](https://github.com/PeanutButterRat/hasher)

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ACKNOWLEDGMENTS -->
## Acknowledgments

Here are some awesome resources that were crucial to the development to this application.

* [Clap](https://github.com/clap-rs/clap)
* [SHA-2 Specification](https://csrc.nist.gov/files/pubs/fips/180-2/final/docs/fips180-2.pdf)
* [Hash Checker](https://emn178.github.io/online-tools/sha256.html)
* [README Template](https://github.com/othneildrew/Best-README-Template)
* [README Table Generator](https://www.tablesgenerator.com/markdown_tables)
<p align="right">(<a href="#readme-top">back to top</a>)</p>


[license-shield]: https://img.shields.io/github/license/othneildrew/Best-README-Template.svg?style=for-the-badge
[license-url]: https://github.com/PeanutButterRat/hasher/LICENSE
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://www.linkedin.com/in/eric-brown-b0a258202/
