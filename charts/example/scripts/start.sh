k3d_start() {
  echo "------------------"
  echo "Create k3d cluster"
  echo "------------------"
  k3d cluster create --config ./charts/example/scripts/k3d-config.yml
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
