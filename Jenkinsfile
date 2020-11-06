// Uses Declarative syntax to run commands inside a container.
pipeline {
    triggers {
        pollSCM("*/5 * * * *")
    }
    agent {
        kubernetes {
            yaml '''
apiVersion: v1
kind: Pod
spec:
  imagePullSecrets:
    - name: dev-imanuel-jenkins-regcred
  containers:
  - name: rust
    image: registry.imanuel.dev/rust:latest
    command:
    - sleep
    args:
    - infinity
'''
            defaultContainer 'rust'
        }
    }
    stages {
        stage('Lint code') {
            steps {
                sh 'rustup component add clippy'
                try {
                    clippyOut = sh returnStdout: true, script: 'cargo clippy -- -D warnings'
                } catch (Exception e) {
                    mail bcc: '', body: 'The build of Jinya UI contains errors, please check.\r\n' + e.toString(), cc: '', from: 'noreply@imanuel.dev', replyTo: '', subject: '[jinya-ui] Errors in clippy check', to: 'developers@jinya.de'
                }
            }
        }
    }
}
