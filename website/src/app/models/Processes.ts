export interface ProcessesModel {
    processes: Process[]
  }

export interface Process {
    id: number,
    name: String,
    status: String,
}