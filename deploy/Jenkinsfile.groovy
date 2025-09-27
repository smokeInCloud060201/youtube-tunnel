pipeline {
    agent any

    environment {
        IMAGE_NAME = "my-spring-app"
        IMAGE_TAG  = "${env.BUILD_NUMBER}"
    }

    stages {
        stage('Checkout') {
            steps {
                checkout scm
            }
        }

        stage('Build images') {
            parallel {
                stage('api') {
                    steps {
                        sh "make build-api-image"
                    }
                }
                stage('worker') {
                    steps {
                        sh "make build-worker-image"
                    }
                }
                stage('web') {
                    steps {
                        sh "make build-web-image"
                    }
                }
            }
        }

        stage('Deploy') {
            steps {
                sh """
                    make create-network
                    make deploy-common
                    make deploy-service
                """
            }
        }
    }

    post {
        success {
            echo "Deployment successful"
        }
        failure {
            echo "Deployment failed"
        }
    }
}
