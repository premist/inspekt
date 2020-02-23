inspekt
=======

Inspekt is a simple web service that shows Kubernetes pod and node information via [Downward API](https://kubernetes.io/docs/tasks/inject-data-application/environment-variable-expose-pod-information/#the-downward-api), built with Rust.

![Inspekt](https://user-images.githubusercontent.com/291078/75115160-728f3c80-569f-11ea-8b7c-1198c9be219d.png)


### Up and running

Use [pre-built yamls](deploy/) for deployment to Kubernetes.

```bash
kubectl apply -f https://raw.githubusercontent.com/premist/inspekt/master/deploy/k8s-deployment.yaml
kubectl apply -f https://raw.githubusercontent.com/premist/inspekt/master/deploy/k8s-service.yaml
```

Alternatively, you can use [Docker image](https://hub.docker.com/repository/docker/premist/inspekt) directly, although it is not recommended as environment variables need to be provided via Downward API.


### License

MIT
