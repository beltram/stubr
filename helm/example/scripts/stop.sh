k3d_stop() {
  echo "--------------------------"
  echo "Uninstall stubr Helm chart"
  echo "--------------------------"
  helm uninstall stubr
  echo "----------------"
  echo "Stop k3d cluster"
  echo "----------------"
  k3d cluster stop stubr
  echo "------------------"
  echo "Delete k3d cluster"
  echo "------------------"
  k3d cluster delete stubr
}
