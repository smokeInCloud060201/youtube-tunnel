pipeline {
    agent any

    environment {
        IMAGE_NAME = "my-spring-app"
        IMAGE_TAG  = "${env.BUILD_NUMBER}"
        YOUTUBE_API_KEY = credentials("YOUTUBE_API_KEY")
        YOUTUBE_BASE_HOST = credentials("YOUTUBE_BASE_HOST")
        MINIO_INTERNAL_URL = credentials("MINIO_INTERNAL_URL")
        MINIO_EXTERNAL_URL = credentials("MINIO_EXTERNAL_URL")
        MINIO_USERNAME = credentials("MINIO_USERNAME")
        MINIO_PASSWORD = credentials("MINIO_PASSWORD")
    }

    stages {
        stage('Checkout') {
            steps {
                checkout scm
            }
        }

        stage('Clean images') {
            steps {
                sh 'make clean-app-images'
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
                sh '''
                    cat > .env <<EOF
                    YOUTUBE_API_KEY=${YOUTUBE_API_KEY}
                    YOUTUBE_BASE_HOST=${YOUTUBE_BASE_HOST}
                    MINIO_INTERNAL_URL=${MINIO_INTERNAL_URL}
                    MINIO_EXTERNAL_URL=${MINIO_EXTERNAL_URL}
                    MINIO_USERNAME=${MINIO_USERNAME}
                    MINIO_PASSWORD=${MINIO_PASSWORD}
                    EOF
                '''
                sh 'make deploy'
            }
            post {
                always {
                    sh 'rm -f .env'
                }
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
        always {
            sh 'docker image prune -f'
        }
    }
}
