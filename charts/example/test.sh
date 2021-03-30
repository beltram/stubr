source ./charts/example/scripts/start.sh
source ./charts/example/scripts/stop.sh

echo "--------------------"
echo "Stopping k3s cluster"
echo "--------------------"
k3d_stop
echo "--------------------"
echo "Starting k3s cluster"
echo "--------------------"
k3d_stop
k3d_start

echo "-------------------------------------------------------"
echo "Installing ingress, exposing stubr service on port 8081"
echo "-------------------------------------------------------"
kubectl apply -f charts/example/scripts/ingress.yaml
echo "----------------------"
echo "Installing stubr chart"
echo "----------------------"
helm install --repo https://beltram.github.io/stubr/ hello-stubr stubr
echo "--------------------------"
echo "Make sure pod is installed"
echo "--------------------------"
kubectl get pod -n default

echo "Waiting ..."
sleep 60

echo "--------------------------------------"
echo "calling deployed pod on localhost:8081"
echo "--------------------------------------"
curl http://localhost:8081/stubr

echo "--------------------"
echo "Stopping k3s cluster"
echo "--------------------"
#k3d_stop
