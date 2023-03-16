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
docker build -t rmd_course .
docker run -p 8000:8000 rmd_course

```

## Output Demo
<img width="1088" alt="Screen Shot 2023-03-16 at 7 43 59 PM" src="https://user-images.githubusercontent.com/60382493/225776412-0ea2d46b-2c31-497b-8ac5-6f36be647082.png">
