# Deploying the AKS Store Demo app the Azure using Azure Developer CLI

Using the [Azure Developer CLI](https://learn.microsoft.com/azure/developer/azure-developer-cli/overview), you can deploy this entire solution to Azure and optionally deploy certain Azure services such as Azure Service Bus and Azure Cosmos DB instead of RabbitMQ and MongoDB.

## Prerequisites

[![Open in GitHub Codespaces](https://github.com/codespaces/badge.svg)](https://github.com/codespaces/new?hide_repo_select=true&ref=main&repo=648726487)

Opening the [AKS Store Demo repo](https://github.com/Azure-Samples/aks-store-demo) in [GitHub Codespaces](https://github.com/features/codespaces) is preferred; however, if you want to run the app locally, you will need the following tools:

- [Azure CLI](https://learn.microsoft.com/cli/azure/what-is-azure-cli)
- [Azure Developer CLI](https://learn.microsoft.com/azure/developer/azure-developer-cli/overview)
- [Visual Studio Code](https://code.visualstudio.com/)
- [Docker Desktop](https://www.docker.com/products/docker-desktop/)
- [kubectl](https://kubernetes.io/docs/tasks/tools/)
- [Helm](https://helm.sh/)
- [Git](https://git-scm.com/)
- [Terraform](https://www.terraform.io/)
- [Visual Studio Code](https://code.visualstudio.com/)
- Bash shell

## Get started

To get started, authenticate to Azure using the Azure Developer CLI and Azure CLI.

```bash
# authenticate to Azure Developer CLI
azd auth login

# authenticate to Azure CLI
az login
```

> [!WARNING]
> Before you run the `azd up` command, make sure that you have the "Owner" role on the subscription you are deploying to. This is because the infrastructure-as-code templates will create Azure role based access control (RBAC) assignments. Otherwise, the deployment will fail.

When selecting an Azure region, make sure to choose one that supports all the services used in this app including Azure OpenAI, Azure Kubernetes Service, Azure Key Vault, Azure Service Bus, Azure CosmosDB, Azure Log Analytics Workspace, Azure Monitor workspace, and Azure Managed Grafana.

If you are deploying an Azure OpenAI account, you will need to ensure you have enough [tokens per minute quota](https://learn.microsoft.com/azure/ai-services/openai/how-to/quota?tabs=cli) for the `gpt-35-turbo` model. You can check your quota by running the following command:

```bash
REGION=eastus2

az cognitiveservices usage list \
  --location $REGION \
  --query "[].{name: name.value, currentValue:currentValue, limit: limit}" \
  -o table
```

> [!TIP]
> If difference between current value and limit for `OpenAI.Standard.gpt-35-turbo` is less than 30, you can request more by following the instructions in the [Azure OpenAI documentation](https://learn.microsoft.com/azure/ai-services/openai/quotas-limits#how-to-request-increases-to-the-default-quotas-and-limits).

### Deployment settings

The infrastructure-as-code templates in this repo use variables to define the deployment settings. You can set these variables using the Azure Developer CLI and the templates will evaluate them to provision the resources.

The following environment variables are used to define the deployment settings:

| Variable | Description |
|----------|-------------|
| `DEPLOY_AZURE_CONTAINER_REGISTRY` | By default, all application containers will be sourced from the [GitHub Container Registry](https://github.com/orgs/Azure-Samples/packages?repo_name=aks-store-demo). If you want to deploy apps from an Azure Container registry instead, set this environment variable to `true` to provision an Azure Container Registry and enable authentication from the AKS cluster. When this is set to true, you also have an option to set `BUILD_CONTAINERS` to `true` to build containers from source using the `az acr build command`; otherwise, the containers will be imported from the [GitHub Container Registry](https://github.com/orgs/Azure-Samples/packages?repo_name=aks-store-demo) using the `az acr import` command. |
| `DEPLOY_AZURE_WORKLOAD_IDENTITY` | Set to `true` to deploy Azure Managed Identities for services that support it and enables workload identity and OIDC Issuer URL on AKS. |
| `DEPLOY_AZURE_OPENAI` | Set to `true` to deploy Azure OpenAI, the `ai-service` microservice with workload identity authentication if that option was set to true. |
| `DEPLOY_AZURE_SERVICE_BUS` | Set to `true` to deploy Azure Service Bus and configures workload identity if that option is set to true. |
| `DEPLOY_AZURE_COSMOSDB` | The `makeline-service` supports both MongoDB and SQL API for accessing data in Azure CosmosDB. The default API is `MongoDB`, but you can use SQL API when using Azure CosmosDB. Set this to `true` to deploy Azure Cosmos DB. When this is set to true, you can also set `AZURE_COSMOSDB_ACCOUNT_KIND` to `GlobalDocumentDB` to use the SQL API for Azure Cosmos DB; otherwise, MongoDB API will be used. |
| `DEPLOY_OBSERVABILITY_TOOLS` | Set to `true` to deploy Azure Log Analytics workspace, Azure Monitor managed service for Promethues, Azure Managed Grafana, and onboard the AKS cluster to Container Insights. |

These environment variables listed above can be set with commands like this:

```bash
# deploys azure container registry and builds containers from source
azd env set DEPLOY_AZURE_CONTAINER_REGISTRY true
azd env set BUILD_CONTAINERS true

# enables workload identity on the aks cluster and deploys managed identities
azd env set DEPLOY_AZURE_WORKLOAD_IDENTITY true

# deploys azure openai
azd env set DEPLOY_AZURE_OPENAI true

# deploys azure service bus
azd env set DEPLOY_AZURE_SERVICE_BUS true

# deploys azure cosmos db with the sql api
azd env set DEPLOY_AZURE_COSMOSDB true
azd env set AZURE_COSMOSDB_ACCOUNT_KIND GlobalDocumentDB

# deploys aks observability tools
azd env set DEPLOY_OBSERVABILITY_TOOLS true
```

> [!NOTE]
> If none of these environment variables are set, only the AKS cluster and Azure Key Vault will be deployed.

## Deploy the app

Provision and deploy the app with a single command.

```bash
azd up
```

When you run the `azd up` command for the first time, you will be asked for a bit of information:

- **Environment name:** This is the name of the environment that will be created so that Azure Developer CLI can keep track of the resources that are created.
- **Azure subscription:** You will be asked to select the Azure subscription that you want to use. If you only have one subscription, it will be selected by default.
- **Azure location:** You will be asked to select the Azure location where the resources will be created. You can select the location that is closest to you but you must ensure that the location supports all the resources that will be created. If you are unsure of which region to use, select "East US 2".

After you have provided the information, the `azd up` command will start by registering Azure providers, features, and installing Azure CLI extensions. From there, it will invoke the `terraform apply` command, then execute "azd-hook" scripts, which is a neat way for you to "hook" into the deployment process and add any customizations. In our deployment, we will invoke a `helm install` command to apply our Kubernetes manifests.

This will take a few minutes to complete.

> [!NOTE]
> This deployment will use [Terraform](../infra/terraform) by default, but you can use [Azure Bicep](../infra/bicep) to provision the Azure resources. To provision the Azure resources using Bicep instead of Terraform, you can rename the `azure-bicep.yaml` file to `azure.yaml` and run the `azd up` command.

## Validate the deployment

Once the deployment is complete, you should see a list of outputs that show the resources that were created. Make a note of the value for `AZURE_RESOURCEGROUP_NAME`. Open the [Azure Portal](https://portal.azure.com), and navigate to the resource group. You should see an AKS cluster. Click on the AKS resource to open it. In the Kubernetes resources section, click on the Workloads tab. You will see the application deployments in the pets namespace. Next, click on the Services and ingresses tab. You will see the Kubernetes Services that are deployed in your Kubernetes cluster. For the store-admin and store-front services, you'll notice that the Type is LoadBalancer. This means that the services are exposed to the internet via public IP addresses. You can click on the External IP to open the app in your browser.

If you deployed an Azure Service Bus, navigate to the resource and use Azure Service Bus explorer to check for order messages.

If you deployed an Azure CosmosDB, navigate to the resource and use the database explorer to check for order records.

## Clean up

When you are done testing the deployment, you can clean up the resources using the `azd down` command.

```bash
azd down --force --purge
```