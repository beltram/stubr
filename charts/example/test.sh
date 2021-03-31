source ./charts/example/scripts/start.sh
source ./charts/example/scripts/stop.sh

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
kubectl apply -f charts/example/scripts/ingress.yaml
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