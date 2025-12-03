import { Agent, AgentInput, AgentOutput } from '../agent';

export class ProjectManagerAgent implements Agent {
  name = 'ProjectManager';
  role = 'Project Manager Agent';
  systemPrompt = `
    You are the Project Manager Agent for bl1nk/BLinkOS.
    Your job is to:
    1. Break down requirements into tasks
    2. Assign tasks to appropriate agents
    3. Track progress
    4. Identify blockers
  `;

  async execute(input: AgentInput): Promise<AgentOutput> {
    try {
      const { task } = input.data;

      // ตัวอย่าง: แตกงานใหญ่เป็นงานย่อย
      const subtasks = this.breakDownTask(task);

      return {
        success: true,
        data: {
          originalTask: task,
          subtasks,
          assignments: this.assignTasks(subtasks)
        },
        timestamp: new Date().toISOString(),
        agentName: this.name
      };
    } catch (error) {
      return {
        success: false,
        error: String(error),
        timestamp: new Date().toISOString(),
        agentName: this.name
      };
    }
  }

  private breakDownTask(task: any): any[] {
    // ตัวอย่างการแตกงาน
    return [
      {
        id: `${task.id}-1`,
        title: `Define: ${task.title}`,
        type: 'definition',
        owner: 'Architecture'
      },
      {
        id: `${task.id}-2`,
        title: `Implement: ${task.title}`,
        type: 'implementation',
        owner: 'Reviewer'
      },
      {
        id: `${task.id}-3`,
        title: `Document: ${task.title}`,
        type: 'documentation',
        owner: 'DocumentWriter'
      }
    ];
  }

  private assignTasks(subtasks: any[]): Record<string, any> {
    const assignments: Record<string, any> = {};
    
    subtasks.forEach(subtask => {
      if (!assignments[subtask.owner]) {
        assignments[subtask.owner] = [];
      }
      assignments[subtask.owner].push(subtask);
    });

    return assignments;
  }
}
