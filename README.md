# Beibei-Du-IDS721_Project2
Kubernetes based Continuous Delivery Create a customized Docker container
- Create a customized Docker container from the current version of Python that deploys a simple python script.
- Push image to DockerHub, or Cloud based Container Registery (ECR)
- Project should deploy automatically to Kubernetes cluster
- Deployment should be to some form of Kubernetes service (can be hosted like Google Cloud Run or Amazon EKS, etc)

## Planing
1. Decided a topic: Recommender System on Cousera courses
2. Watch tutorials:
3. Set-ups: Makefile, CI/CD, virtual environments, etc
4. Coding part
5. Deployment
* Planing doc: [doc] {https://docs.google.com/document/d/1SMkQSX9lCwID7ROtmD5sWQYg53w-REIM2b3GPReCgN4/edit}

## Rust Setup
```
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
source "$HOME/.cargo/env"
cargo new ids721_proj2_cousera
```
## Docker Image
```
docker build -t my_app .
docker run -p 8000:8000 my_app

```
