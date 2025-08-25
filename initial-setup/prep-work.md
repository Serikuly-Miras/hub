# Prep Work: Ubuntu Server Setup

## 1. Update and Upgrade Packages

```sh
sudo apt update
sudo apt upgrade -y
```

## 2. Disable Swap

```sh
sudo swapoff -a
sudo sed -i '/ swap / s/^/#/' /etc/fstab
```

## 3. Enable Automatic Security Updates

```sh
sudo apt update
sudo apt install -y unattended-upgrades
sudo dpkg-reconfigure --priority=low unattended-upgrades
```

## 4. Set Up UFW Firewall

```sh
sudo apt install -y ufw
sudo ufw default deny incoming
sudo ufw default allow outgoing
sudo ufw allow OpenSSH
sudo ufw allow http
sudo ufw allow https
sudo ufw enable
sudo ufw status
```

## 5. Install PostgreSQL

```sh
sudo apt update
sudo apt install -y postgresql postgresql-contrib
sudo systemctl enable postgresql
sudo systemctl start postgresql
# Proceed to set up PostgreSQL as needed
```

## 6. Install k3s (Single Node)

```sh
# See: https://docs.k3s.io/quick-start
curl -sfL https://get.k3s.io | sh -
sudo kubectl get nodes
```

## 7. Install Helm

```sh
curl -fsSL -o get_helm.sh https://raw.githubusercontent.com/helm/helm/main/scripts/get-helm-3
chmod 700 get_helm.sh
./get_helm.sh
```

## 8. Install k9s

```sh
#https://github.com/derailed/k9s
wget https://github.com/derailed/k9s/releases/latest/download/k9s_linux_amd64.deb && apt install ./k9s_linux_amd64.deb && rm k9s_linux_amd64.deb
```
