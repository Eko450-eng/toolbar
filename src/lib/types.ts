export type Note = {
	id?: number;
	created_at?: Date;
	title: string;
	note: string;
	folder: string;
};

export type NoteWithTasks = {
	note_id: number;
	title: string;
	tasks: TaskItem[];
};

export type TaskItem = {
	checked: boolean;
	label: string;
};

export const initNote: Note = {
	id: -10,
	note: '',
	folder: '',
	title: '',
};
