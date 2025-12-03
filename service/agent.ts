// Agent Base Interface
export interface Agent {
  name: string;
  role: string;
  systemPrompt: string;
  execute(input: AgentInput): Promise<AgentOutput>;
}

export interface AgentInput {
  type: string;
  data: Record<string, any>;
  context?: Record<string, any>;
}

export interface AgentOutput {
  success: boolean;
  data?: any;
  error?: string;
  timestamp: string;
  agentName: string;
}

export interface Task {
  id: string;
  title: string;
  description: string;
  status: 'pending' | 'in-progress' | 'review' | 'done';
  assignedAgent: string;
  priority: 'high' | 'medium' | 'low';
  dueDate?: string;
  dependencies?: string[];

}
