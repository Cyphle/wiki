# Test avec Terraform

* Aller voir le repository Terratest pour des exemples de tests et des outils de test

## Module self validation
### Validations
* Validation bloc sur les inputs. Attention cela ne permet que de vérifier l'input courant et il n'est pas possible de faire référence à d'autre élément.
```
variable "instance_type" {
    description = "The type of EC2 Instances to run (e.g. t2.micro)"
    type = string

    validation {
        condition = conttains(["t2.micro", "t3.micro"], var.instance_type)
        error_message = "Only free tier is allowed: t2.micro | t3.micro."
    }

    validation {
        condition = var.min_size <= 10
        error_message = "ASGs must have 10 or fewer instances to keep costs down."
    }
}
```

### Preconditions & postconditions
* Preconditions are for catching errors before running apply
```
data "aws_ec2_instance_type" "instance" {
    instance_type = var.instance_type
}

resource "aws_launch_configuration" "example" {
    image_id = var.ami
    instance_type = var.instance_type
    security_groups = [aws_security_group.instance.id]
    user_data = var.user_data

    # Required when using a launch configuration with an auto scaling group
    lifecycle {
        create_before_destroy = true
        precondition {
            condition = data.aws_ec2_instance_type_instance_free_tier_eligible
            error_message = "${var.instance_type} is not part of the AWS Free tier !"
        }
    }
}
```
* Postconditions are for catching errors after running apply. Par exemple pour check qu'un ASG a été deployé cross AZ
```
resource "aws_autoscaling_group" "example" {
    name = var.cluster_name
    launch_configuration = aws_launch_configuration.example.name
    vpc_zone_identifier = var.subnet_ids

    lifecycle {
        postcondition {
            # Keyword self can only be used in postconditions to refer to output
            condition = length(self.availability_zones) > 1
            error_message = "You must use more than one AZ for high availability!"
        }
    }
}
```

### When to use validations, preconditions & postconditions
* Validation for basic input sanitization
* Precondition for basic asumptions
* Postcondition for basic guarantees
* Automated tests for other advanced checks

## Automated tests
* Il n'existe pas de tests unitaire Terraform. Ce sont forcément des tests d'intégration.
* Stratégie:
    1. Create a small standalone module
    2. Create easy to deploy example
    3. Run `terraform apply`
    4. Validate
    5. Run `terraform destroy`
* Une bonne pratique est d'avoir des comptes AWS dédiés aux tests pour pas impacter les infras de prod
* Pour effectuer les tests et notamment les validations, il faut s'appuyer sur un language de dév
* La librairie Terratest, en Go, permet d'effectuer des tests d'IaC (Terraform, Packer, Docker, Helm) au sein de cloud (AWS, Google, Kubernetes, ...)
* Il faut utiliser les namespaces Terraform pour lancer des tests afin d'éviter le conflit avec les noms de ressources dans le cas où des tests utilisent les mêmes noms
* Si plusieurs tests du même module sont lancés en parallèle, il va y avoir un conflit de terraform init. Pour cela, il faut copier le module dans un dossier temporaire pour chaque test. Terratest possède le helper `test_structure.CopyTerraformFolderToTemp`

### Unit test example in Go
```
package test

import (
	"fmt"
	"github.com/gruntwork-io/terratest/modules/http-helper"
	"github.com/gruntwork-io/terratest/modules/terraform"
	"testing"
)

func TestAlbExample(t *testing.T) {
    // For parallel run of tests
    t.Parallel()

	opts := &terraform.Options{
		TerraformDir: "../examples/networking/alb",

        // Inputs (variables) of module
        Vars: map[string]interface{}{
			"alb_name": fmt.Sprintf("test-%s", random.UniqueId())
		}
	}

	// Clean up after test. defer makes sure that function is called however function exits
	defer terraform.Destroy(t, opts)

	// Deploy the example
	terraform.InitAndDeploy(t, opts)

	// Get the URl of the ALB (output of the module)
	albDnsName := terraform.OutputRequired(t, opts, "alb_dns_name")
	url := fmt.Sprintf("http://%s", albDnsName)

	// Test that the ALB's default action is working and returns a 404
	expectedStatus := 404
	expectedBody := "404: page not found"
	maxRetries := 10
	timeBetweenRetries := 10 * time.Second

	http_helper.HttpGetWithRetry(
		t,
		url,
		nil,
		expectedStatus,
		expectedBody,
		maxRetries,
		timeBetweenRetries
	)
}
```

### Integration test example in go
```
package test

import (
	"fmt"
	"github.com/gruntwork-io/terratest/modules/http-helper"
	"github.com/gruntwork-io/terratest/modules/terraform"
	"testing"
)

const dbDirStage = "../../live/stage/data-stores/mysql"
const appDirStage = "../../live/stage/services/hello-world-app"

func TestHelloWorldAppStage(t *testing.T) {
	t.Parallel()

	// Deploy the MySQL DB
	dbOpts := createDbOpts(t, dbDirStage)
	defer terraform.Destroy(t, dbOpts)
	terraform.InitAndDeploy(t, dbOpts)

	// Deploy the hello-world-app
	helloOpts := createHelloOpts(dbOpts, appDirStage)
	defer terraform.Destroy(t, helloOpts)
	terraform.InitAndDeploy(t, helloOpts)

	// Validate the hello-world-app works
	validateHelloApp(t, helloOpts)
}

func createDbOpts(t *testing.T, terraformDir string) *terraform.Options {
	uniqueId := random.uniqueId()

	bucketForTesting := "YOUR_S3_BUCKET_FOR_TESTING"
	bucketRegionForTesting := "YOUR_S3_BUCKET_REGION_FOR_TESTING"
	dbStateKey := fmt.Sprintf("%s/%s/terraform.tfstate", t.Name(), uniqueId)

	return &terraform.Options{
		terraformDir: terraformDir,

		Vars: map[string]interface{}{
			"db_name": fmt.Sprintf("test%s", uniqueId)
			"db_username": "admin"
			"db_password": "password"
		}

		// Define backend for testing
		BackendConfig: map[string]interface{}{
			"bucket": bucketForTesting,
			"region": bucketRegionForTesting,
			"key": dbStateKey,
			"encrypt": true
		}
	}
}

func createHelloOpts(dbOpts *terraform.Options, terraformDir string) *terraform.Options {
	return &terraform.Options{
		TerraformDir: terraformDir,

		Vars: map[string]interface{}{
			"db_remote_state_bucket": dbOpts.BackendConfig["bucket"],
			"db_remote_state_key": dbOpts.BackendConfig["key"],
			"environment": dbOpts.Vars["db_name"]
		}
	}
}

func validateHelloApp(t *testing.T, helloOpts *terraform.Options) {
	albDnsName := terraform.OutputRequired(t, helloOpts, "alb_dns_name")
	url := fmt.Sprintf("http://%s", albDnsName)

	maxRetries := 10
	timeBetweenRetries := 10 * time.Second

	http_helper.HttpGetWithRetryWithCustomValidation(
		t,
		url,
		nil,
		maxRetries,
		timeBetweenRetries,
		func(status int, body string) bool {
			return status == 200 && strings.Contains(body, "Hello, World")
		}
	)
}
```

## Test stages
* Cela permet de ne redéployer qu'une partie de l'infra définie dans les tests. Par exemple, si un test d'intégration inclue une base de donnée et une application, cela rajoute de l'overhead de devoir redéployer la base de donnée à chaque changement sur l'application.
* Terratest permet de définir des stages à lancer ou pas avec `test_structure` package.
* Définition d'un stage `stage(t, "<stage_name>", func() { <function_to_execute_at_stage> })`
* Pour skip un stage `SKIP_<stage_name>=true` (il s'agit d'une variable d'environnement). Example
```
SKIP_teardown_db=true SKIP_teardown_app=true go test -timeout 30m -run 'TestHelloWorldAppStageWithStages'
```

### Example de test with stages
```
// With test stages
func TestHelloWorldAppStageWithStages(t *testing.T) {
	t.Parallel()

	stage := test_structure.RunTestStage

	// Deploy the MySQL DB
	defer stage(t, "teardown_db", func() { teardownDb(t, dbDirStage) })
	stage(t, "deploy_db", func() { deployDb(t, dbDirStage) })

	// Deploy the hello-world-app
	defer stage(t, "teardown_app", func() { teardownApp(t, appDirStage) })
	stage(t, "deploy_app", func() { deployApp(t, dbDirStage, appDirStage) })

	// Validate the hello-world-app works
	stage(t, "validate_app", func() { validateApp(t, appDirStage) })
}

func deployDb(t *testing.T, dbDir string) {
	dbOpts := createDbOpts(t, dbDir)

	// Save data to disk so that other test stages executed at a later time can read the data back in
	test_structure.SaveTerraformOptions(t, dbDir, dbOpts)

	terraform.InitAndDeploy(t, dbOpts)
}

func teardownDb(t *testing.T, dbDir string) {
	dbOpts := test_structure.LoadTerraformOptions(t, dbDir)
	defer terraform.Destroy(t, dbOpts)
}

func deployApp(t *testing.T, dbDir string, helloAppDir string) {
	dbOpts := test_structure.LoadTerraformOptions(t, dbDir)
	helloOpts := createHelloOpts(dbOpts, helloAppDir)

	// Save data to disk so that other test stages executed at a later time can read the data back in
	test_structure.SaveTerraformOptions(t, helloAppDir, helloOpts)

	terraform.InitAndDeploy(t, helloOpts)
}

func teardownApp(t *testing.T, helloAppDir string) {
	helloOpts := test_strcture.LoadTerraformOptions(t, helloAppDir)
	defer terraform.Destroy(t, helloOpts)
}

func validateApp(t *testing.T, helloAppDir string) {
	helloOpts := test_structure.LoadTerraformOptions(t, helloAppDir)
	validateHelloApp(t, helloOpts)
}
```

## Retries
* Parfois les tests vont fail à cause de raisons dues à l'infrastructure (une instance EC2 qui ne se déploie pas, un problème de TLS handshake, ...)
* Il est possible de spécifier des retries avec Terratest avec `MaxRetries`, `TimeBetweenReties` et `RetryableTErraformErrors`
```
func createHelloOpts(dbOpts *terraform.Options, terraformDir string) *terraform.Options {
	return &terraform.Options{
		TerraformDir: terraformDir,

		Vars: map[string]interface{}{
			"db_remote_state_bucket": dbOpts.BackendConfig["bucket"],
			"db_remote_state_key": dbOpts.BackendConfig["key"],
			"environment": dbOpts.Vars["db_name"]
		}

		// Retry up to 3 times, with 5 seconds between retries on known errors
		maxRetries: 3,
		timeBetweenRetries: 5 * time.Second,
		RetryableTerraformErrors: map[string]string{
            // Map error message to look into to additional information to display
			"RequestError: send request failed": "Throttling issue?"
		}
	}
}
```

## End-to-end tests
* Stratégie :
    * Monter un environnement entier dans un environnement de test et le conserver
    * Ne déployer que les modifications
    * Appliquer des tests e2e sur cet environnement

## Other testing strategies
### Static analysis
* Parse the code and analyze it without executing it
* Quelques outils: terraform validate, tfsec, tflint, Terrascan
* Example `terraform validate` pour check si une variable a été oubliée
* Certains outils permettent des validations plus avancées comme
    * Les security groups ne peuvent pas être trop ouverts
    * Tous les EC2 doivent avoir certains tags

### Plan testing
* Run `terraform plan` et analyser la sortie
* Quelques outils: Terratest, Open Policy Agent (OPA), HishiCorp Sentinel, Checkov, terraform-compliance

#### Exemple avec terratest
```
func TestAlbExamplePlan(t *testing.T) {
	t.Parallel()

	albName := fmt.Sprintf("test-%s", random.UniqueId())

	opts := @terraform.Options{
		TerraformDir: "../examples/alb"
		Vars: map[string]interface{}{
			"alb_name": albName
		}
	}

	// One version
	planString := terraform.InitAndDeploy(t, opts)

	resourceCounts := terraform.GetResourceCount(t, planString)
	require.Equal(t, 5, resourceCounts.Add)
	require.Equal(t, 0, resourceCounts.Change)
	require.Equal(t, 0, resourceCounts.Destroy)

	// A second version which gives access to all resources that have to be deployed
	planStruct := terraform.InitAndPlanAndShowWithStructNoLogTempPlanFile(t, opts)

	alb, exists := planStruct.ResourcePlannedValuesMap["module.alb.aws_lb.example"]
	require.True(t, exists, "aws_lb resource must exists")
	name, exists := alb.AttributeValues["name"]
	require.True(t, exists, "missing name parameter")
	require.Equal(t, albName, name)
}
```

#### Exemple avec Open Policy Agent (OPA)
```
// enforce_tagging.rego

package terraform

allow {
    resource_change := input.resource_changes[_]
    // Checks that every resource has tag "ManagedBy"
    resource_change.change.after.rags["ManagedBy"]
}

// Then run
terraform plan -out tfplan.binary
terraform show -json tfplan.binary > tfplan.json
opa eval --data enforce_tagging.rego --input tfplan.json --format pretty data.terraform.allow
```

### Server testing
* Validate servers configurations
* Quelques outils : InSpec, Serverspec, Goss

#### Exemple avec InSpec
* Validate permissions on specific files, has certain dependencies and listing on some ports
```
describe file('/etc/myapp.conf') do
    it { should exist }
    its('mode') { should cmp 0644 }
end

describe apache_conf do
    its('Listen') { should cmp 8080 }
end

describe port(8080) do
    it { should be_listening }
end
```