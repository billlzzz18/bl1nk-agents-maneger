import express, { Express, Request, Response } from 'express';
import cors from 'cors';
import { AgentOrchestrator } from './orchestrator/agent-runner';
import { ProjectManagerAgent } from './agents/project-manager';
import { Task, WorkflowState } from './agent';

const app: Express = express();
const PORT = process.env.PORT || 3000;

// Middleware
app.use(cors());
app.use(express.json());

// Initialize Orchestrator
const orchestrator = new AgentOrchestrator();
orchestrator.registerAgent(new ProjectManagerAgent());

// Routes

// ✅ สร้างงานใหม่
app.post('/api/tasks', (req: Request, res: Response) => {
  const { title, description, priority } = req.body;

  const newTask: Task = {
    id: `task-${Date.now()}`,
    title,
    description,
    status: 'pending',
    assignedAgent: 'ProjectManager',
    priority: priority || 'medium'
  };

  res.json({
    success: true,
    task: newTask,
    message: 'Task created and queued for processing'
  });
});

// ✅ รัน Workflow
app.post('/api/workflow/run', async (req: Request, res: Response) => {
  try {
    const { tasks } = req.body;
    const result = await orchestrator.runWorkflow(tasks);

    res.json({
      success: true,
      workflow: result
    });
  } catch (error) {
    res.status(500).json({
      success: false,
      error: String(error)
    });
  }
});

// ✅ ได้รับสถานะ
app.get('/api/workflow/status', (req: Request, res: Response) => {
  const status = orchestrator.getStatus();

  res.json({
    success: true,
    status
  });
});

// ✅ Health Check
app.get('/api/health', (req: Request, res: Response) => {
  res.json({
    status: 'ok',
    timestamp: new Date().toISOString()
  });
});

// Start Server
app.listen(PORT, () => {
  console.log(`🚀 Agent Orchestrator running on http://localhost:${PORT}`);
});
