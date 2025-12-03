import { Agent, AgentInput, AgentOutput, Task, WorkflowState } from '../agent';

export class AgentOrchestrator {
  private agents: Map<string, Agent> = new Map();
  private workflowState: WorkflowState = {
    tasks: [],
    activeAgents: [],
    completedTasks: [],
    blockers: []
  };

  // ลงทะเบียน Agent
  registerAgent(agent: Agent): void {
    this.agents.set(agent.name, agent);
    console.log(`✅ Agent registered: ${agent.name}`);
  }

  // รัน Workflow
  async runWorkflow(tasks: Task[]): Promise<WorkflowState> {
    this.workflowState.tasks = tasks;

    for (const task of tasks) {
      if (task.status === 'pending') {
        await this.executeTask(task);
      }
    }

    return this.workflowState;
  }

  // รัน Task เดี่ยว
  private async executeTask(task: Task): Promise<void> {
    const agent = this.agents.get(task.assignedAgent);
    
    if (!agent) {
      this.workflowState.blockers.push(
        `❌ Agent not found: ${task.assignedAgent}`
      );
      return;
    }

    try {
      this.workflowState.activeAgents.push(agent.name);
      task.status = 'in-progress';

      const input: AgentInput = {
        type: task.title,
        data: { task },
        context: this.workflowState
      };

      const output = await agent.execute(input);

      if (output.success) {
        task.status = 'done';
        this.workflowState.completedTasks.push(task.id);
      } else {
        task.status = 'review';
        this.workflowState.blockers.push(output.error || 'Unknown error');
      }
    } catch (error) {
      task.status = 'review';
      this.workflowState.blockers.push(`Error: ${error}`);
    } finally {
      this.workflowState.activeAgents = 
        this.workflowState.activeAgents.filter(a => a !== agent.name);
    }
  }

  // ได้รับสถานะปัจจุบัน
  getStatus(): WorkflowState {
    return this.workflowState;
  }
}
