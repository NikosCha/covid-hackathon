# 🥥 cocotrace | Confidential Contact Tracing

This is the repo for the *cocotrace* project in the CodeVsCovid19 Hackathon. 

https://devpost.com/software/cocotrace-confidential-contact-tracing

https://www.codevscovid19.org

Most members of this team are from [decentriq](https://www.decentriq.ch).

## The Problem

Contact tracing has helped drastically flatten the curve in countries such as South Korea. However, tracing requires tracking people's movements. This raises legitimate privacy concerns.

## The Solution

At decentriq ([www.decentriq.ch](http://www.decentriq.ch/)) we work with privacy preserving technologies. We believe that you can have both, contact tracing and data privacy.

We've built a confidential contact tracing system, including an iOS app and a confidential computing server.

Intel SGX technology allows the server to prove its program logic to the app before the app sends data to the server. In particular the server can prove that it cannot leak the user's data.

## 🥥 cocotrace | Confidential Contact Tracing

The cocotrace system uses a central confidential computing server. The server proves its identity to the apps in the form of a signed hash of its code. Additionally, it prevents memory-attacks through memory encryption and isolation.

cocotrace minimizes the privacy risk through provable data confidentiality.

For the server we use Rust and an SGX-embedded webserver depoyed on an Azure Confidential Compute instance. The app was built in react-native.

### Screenshots

![cocotrace Screenshots](docs/Screenshots.png)

## Traditional Systems

Contact tracing systems which are currently in use come with privacy problems. In central approaches all data could potentially be accessed by the authorities. In "edge approaches" infected people's data are exposed. Israel's Hamagen system is such a case, for a detailed comparison see "Comparison with Hamagen" below.

### Comparison to Hamagen

Israel's health ministery recently launched [Hamagen](https://[https://play.google.com/store/apps/details?id=com.hamagen](https://play.google.com/store/apps/details?id=com.hamagen)
), a contact tracing system. The Hamagen app claims that it only processes the user's location data on device. Which is a good first step, however, it fetches the traces of relevant infected people from public government servers. Hence, the location data of infected patients is not protected at all but instead gets pushed to all other users in order to compute locally if they had been close to the infected person. 

Hamagen protects the data privacy of the healthy people and is a clear improvement over a central server collecting all data on an ongoing basis. It however requires the infected people to share all their  movement data with all people in the system. This sharing is likely to prevent some people from revealing their infection.

In terms of privacy, the *cocotrace* system is superior to Hamagen as *cocotrace* protects the privacy of all participants. The only information getting out about you is the fact that you have met one of patients. If we generalise the returned timestamp to only reveal the day when this happened, we argue that *cocotrace* also protects the privacy of infected people really well. 

In *cocotrace* there is nothing preventing people from letting the system know that they have been infected. This improved privacy should bring more users to the system and improve the crucial contact tracing success rate. 


## 🛠️ Build and install


### Backend

TBD

### Mobile App
Currently MacOS as a build machine and iOS as a target is supported. Linux/Windows as well as Android is currently not supported.

#### Requirements

- `node.js`
- `yarn`
- `cocoapods`
- `Xcode` (only for iOS)

#### iOS

- Install dependencies

    ```
    yarn
    cd ios && pod install && cd ..
    ```   

- Start React Native server 

    ```
    yarn run ios
    ```

#### Android

TBD

## Security
cocotrace is currently considered prototype software. Do not deploy it in production, or trust it with sensitive data.

## License

Rust is primarily distributed under the terms of both the MIT license
and the Apache License (Version 2.0), with portions covered by various
BSD-like licenses.

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.