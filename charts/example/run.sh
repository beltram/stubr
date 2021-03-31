k3d_start() {
  echo "------------------"
  echo "Create k3d cluster"
  echo "------------------"
  k3d cluster create stubr --api-port 127.0.0.1:6445 --port '8081:80@loadbalancer' --wait
  echo "-----------------"
  echo "Start k3d cluster"
  echo "-----------------"
  k3d cluster start stubr
  echo "Waiting 2min for cluster nodes to be ready ..."
  sleep 120
  echo "-----------------"
  echo "Verifying........"
  echo "-----------------"
  k3d cluster list
  echo "---------------------------"
  echo "Update local ~/.kube/config"
  echo "---------------------------"
  k3d kubeconfig merge stubr -d -s -u
}

k3d_stop() {
  echo "----------------"
  echo "Stop k3d cluster"
  echo "----------------"
  k3d cluster stop stubr
  echo "------------------"
  echo "Delete k3d cluster"
  echo "------------------"
  k3d cluster delete stubr
}

echo "--------------------------------"
echo "Stopping any running k3s cluster"
echo "--------------------------------"
k3d_stop
echo "--------------------"
echo "Starting k3s cluster"
echo "--------------------"
k3d_start

echo "-------------------------------------------------------"
echo "Installing ingress, exposing stubr service on port 8081"
echo "-------------------------------------------------------"
cat <<EOF | kubectl apply -f -
apiVersion: networking.k8s.io/v1beta1
kind: Ingress
metadata:
  name: stubr
  annotations:
    ingress.kubernetes.io/ssl-redirect: "false"
spec:
  rules:
    - http:
        paths:
          - path: /
            pathType: Prefix
            backend:
              serviceName: hello-stubr
              servicePort: 80
EOF

echo "----------------------"
echo "Installing stubr chart"
echo "----------------------"
helm pull --repo https://beltram.github.io/stubr/ stubr --untar
cp -R charts/example/stubs stubr
helm install hello-stubr ./stubr
echo "--------------------------"
echo "Make sure pod is installed"
echo "--------------------------"
kubectl get pod -n default

echo "Waiting 20s for pod to be up and running ..."
sleep 20

echo "--------------------------------------"
echo "calling deployed pod on localhost:8081"
echo "--------------------------------------"
curl http://localhost:8081/stubr
