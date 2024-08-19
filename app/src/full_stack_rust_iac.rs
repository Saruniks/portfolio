use fallout_ui::components::divider::Divider;
use fallout_ui::components::typography::header::Header;
use fallout_ui::components::typography::{body_text::BodyText, page_header::PageHeader};
use yew::prelude::*;

#[function_component]
pub fn FullStackRustIacPage() -> Html {
    html! {
            <>
                <PageHeader>{"Rust Full Stack IaC"}</PageHeader>

                <BodyText>{"To deploy my applications I use AWS CDK (TypeScript)."}</BodyText>
                <br/>

                <Header class="mb-1">{"Source Step"}</Header>
                <Divider class="mb-4"/>

                <BodyText>{"As a source provider I use either CodeCommit or Github."}</BodyText>
                <BodyText>{"Example of how source step is defined in AWS CDK:"}</BodyText>

                <pre> {
r###"const pipeline = new codepipeline.Pipeline(this, 'ManagedSccachePipeline')
const sourceOutput = new codepipeline.Artifact()

pipeline.addStage({
    stageName: 'Source',
    actions: [new actions.GitHubSourceAction({
        actionName: 'GitHubSource',
        output: sourceOutput,
        owner: 'Saruniks',
        repo: 'my-github-repo',
        branch: 'main',
        oauthToken: cdk.SecretValue.secretsManager('my-github-key'), # github key from AWS SecretsManager
    })]
})"### } </pre>


                <Header class="mb-1">{"Build Step"}</Header>
                <Divider class="mb-4"/>

                <BodyText>{"For build step of CI/CD I use CodeBuild. As CodeBuild doesn't support Rust builds out of the box, I use custom Docker images."}</BodyText>
                <BodyText>{"Example Dockerfile for building Rust Frontend Application:"}</BodyText>

                <pre>{
r###"FROM rust:1.78

RUN apt-get update && \
    apt-get install -y nodejs npm && \ 
    npm install -g tailwindcss

RUN cargo install --locked trunk
RUN rustup target add wasm32-unknown-unknown 
RUN cargo install wasm-bindgen-cli@0.2.64 -f"###
    } </pre>

                <BodyText>{"Example buildspec.yml for building Rust Frontend Application:"}</BodyText>
                <pre>{
r###"version: "0.2"

env:
  shell: bash

phases:
  build:
    commands:
      - cd frontend/builder-ui
      - export TRUNK_BUILD_RELEASE=true
      - trunk build
      - cd dist

artifacts:
  files:
    - '**/*'
  base-directory: 'frontend/builder-ui/dist'"### } </pre>

            <Header class="mb-1">{"Deployment Step"}</Header>
            <Divider class="mb-4"/>

            <BodyText>{"I'm using S3 behind CloudFront for frontend and ElasticBeanstalk for backend:"}</BodyText>

            <pre>{
r###"pipeline.addStage({
    stageName: 'Deploy',
    actions: [
        new actions.S3DeployAction({
            actionName: 'DeployArtifactsToS3',
            bucket: cloudFrontSpa.bucket,
            input: rustBuildFrontend.buildOutput,
            extract: true,
        }),
        new ElasticBeanstalkDeployAction({
            id: 'DeployArtifactsToEb',
            ebsEnvironmentName: ebStack.ebEnv!!,
            ebsApplicationName: ebStack.ebApp!!,
            input: rustBuildBackend.buildOutput
        }),
    ]
})"### } </pre>

            <Header class="mb-1">{"Database"}</Header>
            <Divider class="mb-4"/>

            <BodyText>{"For persistent data I use RDS (PostgreSQL)."}</BodyText>
            <BodyText>{"I store database credentials in AWS Secrets Manager:"}</BodyText>

            <pre>{
r###"const databaseCredentialsSecret = new secretsmanager.Secret(this, 'RdsPostgresCredentials', {
    secretName: 'RdsPostgresCredentials',
    generateSecretString: {
        secretStringTemplate: JSON.stringify({
            username: 'postgres',
        }),
        excludePunctuation: true,
        includeSpace: false,
        generateStringKey: 'password',
    },
})"### } </pre>

            <BodyText>{"Credentials are retrieved by using "} <code>{"aws-sdk-rust"}</code> {" :"}</BodyText>
            <pre>{
r###"let client = aws_sdk_secretsmanager::Client::from_conf(config);
let db_credentials = client
    .get_secret_value()
    .secret_id(secret_id)
    .send()
    .await?;"### } </pre>
            </>
        }
}
