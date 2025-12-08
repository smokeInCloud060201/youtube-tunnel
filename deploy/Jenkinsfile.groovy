pipeline {
    agent any

    environment {
        IMAGE_TAG  = "latest"

        DOCKER_BIN = "docker"
        DOCKER_COMPOSE_BIN = "docker-compose"
        DOCKER_BASE_PATH = "./deploy/docker"
        DOCKER_COMPOSE_BASE_PATH = "./deploy/compose"
        DOCKER_NETWORK_NAME = "yt-network"

        YOUTUBE_API_KEY = credentials("YOUTUBE_API_KEY")
        YOUTUBE_BASE_HOST = credentials("YOUTUBE_BASE_HOST")
        MINIO_CONTAINER_NAME = credentials("MINIO_CONTAINER_NAME")
        MINIO_API_DOMAIN = credentials("MINIO_API_DOMAIN")
        MINIO_ROOT_USER = credentials("MINIO_ROOT_USER")
        MINIO_ROOT_PASSWORD = credentials("MINIO_ROOT_PASSWORD")
    }

    stages {

        stage('Clean Images') {
            steps {
                sh '''
                    ${DOCKER_COMPOSE_BIN} -f ${DOCKER_COMPOSE_BASE_PATH}/app-docker-compose.yml down --rmi all -v --remove-orphans || true
                    ${DOCKER_BIN} rmi -f youtube-tunnel-api:latest youtube-tunnel-worker:latest youtube-tunnel-web:latest || true
                '''
            }
        }

        stage('Build Images') {
            parallel {

                stage('Build API Image') {
                    steps {
                        sh '''
                            ${DOCKER_BIN} rmi -f youtube-tunnel-api:${IMAGE_TAG} || true
                            ${DOCKER_BIN} build \
                                -f ${DOCKER_BASE_PATH}/api.Dockerfile \
                                -t youtube-tunnel-api:${IMAGE_TAG} \
                                ./backend/api
                        '''
                    }
                }

                stage('Build Worker Image') {
                    steps {
                        sh '''
                            ${DOCKER_BIN} rmi -f youtube-tunnel-worker:${IMAGE_TAG} || true
                            ${DOCKER_BIN} build \
                                -f ${DOCKER_BASE_PATH}/worker.Dockerfile \
                                -t youtube-tunnel-worker:${IMAGE_TAG} \
                                ./backend/worker
                        '''
                    }
                }

                stage('Build Web Image') {
                    steps {
                        sh '''
                            ${DOCKER_BIN} rmi -f youtube-tunnel-web:${IMAGE_TAG} || true
                            ${DOCKER_BIN} build \
                                -f ${DOCKER_BASE_PATH}/web.Dockerfile \
                                -t youtube-tunnel-web:${IMAGE_TAG} \
                                ./web
                        '''
                    }
                }
            }
        }

        stage('Deploy') {
            steps {
                sh '''
                    # Generate .env file
                    cat > .env <<EOF
                    YOUTUBE_API_KEY=${YOUTUBE_API_KEY}
                    YOUTUBE_BASE_HOST=${YOUTUBE_BASE_HOST}
                    MINIO_CONTAINER_NAME=${MINIO_CONTAINER_NAME}
                    MINIO_API_DOMAIN=${MINIO_API_DOMAIN}
                    MINIO_ROOT_USER=${MINIO_ROOT_USER}
                    MINIO_ROOT_PASSWORD=${MINIO_ROOT_PASSWORD}
                    EOF
                '''

                sh '''
                    # Create Docker network if not exists
                    ${DOCKER_BIN} network inspect ${DOCKER_NETWORK_NAME} >/dev/null 2>&1 || \
                    ${DOCKER_BIN} network create ${DOCKER_NETWORK_NAME}
                '''

                sh '''
                    # Take down existing services
                    ${DOCKER_COMPOSE_BIN} -f ${DOCKER_COMPOSE_BASE_PATH}/app-docker-compose.yml down || true
                '''

                sh '''
                    # Deploy new version
                    ${DOCKER_COMPOSE_BIN} -f ${DOCKER_COMPOSE_BASE_PATH}/app-docker-compose.yml up -d --build
                '''
            }
            post {
                always {
                    sh 'rm -f .env'
                }
            }
        }

    }

    post {
        success { echo "Deployment successful" }
        failure { echo "Deployment failed" }
    }
}
