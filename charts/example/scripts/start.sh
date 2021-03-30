k3d_start() {
  echo "------------------"
  echo "Create k3d cluster"
  echo "------------------"
  k3d cluster create --config ./helm/test/scripts/k3d-config.yml
  echo "-----------------"
  echo "Start k3d cluster"
  echo "-----------------"
  k3d cluster start stubr
  echo "-----------------"
  echo "Verifying........"
  echo "-----------------"
  k3d cluster list
  echo "---------------------------"
  echo "Update local ~/.kube/config"
  echo "---------------------------"
  k3d kubeconfig merge stubr -d -s -u
}
