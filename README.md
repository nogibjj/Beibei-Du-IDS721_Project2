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

## Kubernetes Setup: minikube
<img width="1205" alt="Screen Shot 2023-03-16 at 8 44 53 PM" src="https://user-images.githubusercontent.com/60382493/225783159-aac9603b-7891-46de-bba7-9cf2e5429ad3.png">

```
curl -LO https://storage.googleapis.com/minikube/releases/latest/minikube-darwin-amd64
sudo install minikube-darwin-amd64 /usr/local/bin/minikube
minikube start
```

## Output Demo
<img width="1088" alt="Screen Shot 2023-03-16 at 7 43 59 PM" src="https://user-images.githubusercontent.com/60382493/225776412-0ea2d46b-2c31-497b-8ac5-6f36be647082.png">
- For instance, if you are currently taking the Cousera course: `Write A Feature Length Screenplay For Film Or Television`, then the top 3 suggested Coursera courses are: `Script Writing: Write a Pilot Episode for a TV or Web Series (Project-Centered Course)`, `Write Your First Novel`, and `Transmedia Writing`, where they have similar content in the courses.

## How to use
- In my Repo, there is a file `make_it_simpler.ipynb`. In order to get the dataset needed for `main.rs`, the dataset is too large to push in Github Repo. The dataset is `cosine_similarities.csv`
```
cosine_similarities = pd.DataFrame(cosine_similarities, index=df['Course Name'], columns=df['Course Name'])
cosine_similarities.head()
cosine_similarities.to_csv('cosine_similarities.csv')
```
## Reference
[1]. https://minikube.sigs.k8s.io/docs/start/
