source ./helm/test/scripts/start.sh
source ./helm/test/scripts/stop.sh

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
kubectl apply -f helm/test/scripts/ingress.yaml
echo "----------------------"
echo "Installing stubr chart"
echo "----------------------"
helm install stubr ./helm
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
