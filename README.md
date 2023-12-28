# Back4app-Rust-Template

This repository demonstrates dockerizing and deploying a [Rust](https://www.rust-lang.org/) application to Back4app Containers.

To learn more, visit this article [How to Deploy a Rust Application With Back4app](https://blog.back4app.com/how-to-deploy-a-rust-application-with-back4app/)

## Dockerize Your Application

1. Install Docker (If you haven’t installed it already)
2. Build the docker image

   ```bash
   docker build -t Rust-API .
   ```

3. Run the docker image as a container

   ```bash
   docker run -p 8000:8000 Rust-API
   ```

4. Navigate to [http://localhost:8000/](HTTP://localhost:8000/) in your web browser to access the application

## Deploy on Back4App

1. Push the application to a GitHub repository
2. Set Up Your Back4app Account
3. Create a Back4app CaaS application

   ![new-back4app-application](assets/new-back4app-application.webp)

4. Link your GitHub account to the Back4app application


5. Select the application’s repository


6. Deploy the application


Your Rust app will now be deployed on the Back4app container platform!
