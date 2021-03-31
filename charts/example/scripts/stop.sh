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
