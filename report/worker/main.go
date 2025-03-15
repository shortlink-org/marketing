package main

import (
	"log"

	"go.temporal.io/sdk/client"
	"go.temporal.io/sdk/worker"

	"github.com/shortlink-org/shortlink/boundaries/marketing/report/activity"
	"github.com/shortlink-org/shortlink/boundaries/marketing/report/shared"
	"github.com/shortlink-org/shortlink/boundaries/marketing/report/workflow"
)

func main() {
	// Create the client object just once per process
	c, err := client.Dial(client.Options{
		Namespace: client.DefaultNamespace,
	})
	if err != nil {
		log.Fatalln("unable to create Temporal client", err)
	}
	defer c.Close()

	// This worker hosts both Workflow and Activity functions
	w := worker.New(c, shared.GreetingTaskQueue, worker.Options{})
	w.RegisterWorkflow(workflow.GreetingWorkflow)
	w.RegisterActivity(activity.ComposeGreeting)

	// Start listening to the Task Queue
	err = w.Run(worker.InterruptCh())
	if err != nil {
		log.Fatalln("unable to start Worker", err)
	}
}
