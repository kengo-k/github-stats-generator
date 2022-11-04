import { Response, Request, Router } from 'express';
import { HttpStatus } from '../models/http-status';
import { Task } from '../models/task';

const router = Router({ mergeParams: true });

const tasks: Task[] = [];

router.get('/', (req: Request, res: Response) => {

    res.send(tasks);
});

router.post('/', (req: Request, res: Response) => {

    const { name, desc } = req.body;

    if (name && desc) {

        const id = tasks.length === 0 ? 1 : tasks[tasks.length - 1].id + 1;

        const task = { id, name, desc };
        tasks.push(task);

        res.status(HttpStatus.Created).send(task);
        return;
    }

    res.sendStatus(HttpStatus.BadRequest);
});

router.put('/:taskId', (req: Request, res: Response) => {

    const { taskId } = req.params;
    const { name, desc } = req.body;

    if (name && desc) {
        const id = parseInt(taskId);
        const task = tasks.find(task => task.id === id);
        if (task) {
            const index = tasks.findIndex(task => task.id === id);
            tasks.splice(index, 1);
            const newTask = { id, name, desc };
            tasks.push(newTask);

            res.status(HttpStatus.OK).send(newTask);
            return;
        }

        res.sendStatus(HttpStatus.NotFound);
        return;
    }

    res.sendStatus(400);
});

router.delete('/:taskId', (req: Request, res: Response) => {

    const { taskId } = req.params;

    if (taskId) {
        const id = parseInt(taskId);
        const task = tasks.find(task => task.id === id);
        if (task) {
            const index = tasks.findIndex(task => task.id === id);
            tasks.splice(index, 1);

            res.sendStatus(HttpStatus.OK);
            return;
        }

        res.sendStatus(HttpStatus.NotFound);
        return;
    }

    res.sendStatus(HttpStatus.BadRequest);
});

export default router;
