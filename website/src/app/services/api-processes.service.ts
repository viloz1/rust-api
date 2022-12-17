import { HttpClient, HttpHeaderResponse, HttpHeaders } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { BehaviorSubject, Observable } from 'rxjs';
import { Process } from '../models/Processes';

@Injectable({
  providedIn: 'root'
})
export class ApiProcessesService {

  constructor(private http: HttpClient) { }

  getProcesses() {
    let header: HttpHeaders = new HttpHeaders();
    return this.http.get("http://localhost:4200/api/processes/get_processes", {headers: header});
  }

  startProcess(id: number) {
    let header: HttpHeaders = new HttpHeaders();
    console.log("start")
    return this.http.post(`http://localhost:4200/api/processes/start/${id}`, {headers: header});
  }

  stopProcess(id: number) {
    let header: HttpHeaders = new HttpHeaders();
    return this.http.post(`http://localhost:4200/api/processes/stop/${id}`, {headers: header});
  }

  restartProcess(id: number) {
    let header: HttpHeaders = new HttpHeaders();
    return this.http.post(`http://localhost:4200/api/processes/restart/${id}`, {headers: header});
  }

  create(name: string, start: string, stop: string, build: string, git: string, branch: string) {
    let header: HttpHeaders = new HttpHeaders();
    let model = {
      name: name,
      start_cmd: start,
      stop_cmd: stop,
      build_cmd: build,
      branch: branch,
      git_url: git,
    }
    return this.http.post(`http://localhost:4200/api/processes/create`, model, {headers: header, responseType: "text"});
  }

  update(id: number) {
    let header: HttpHeaders = new HttpHeaders();
    let model = {
      name: "Hej ts updated7",
      path: "Hej ts updated6",
      start_cmd: "Hej ts updated5",
      stop_cmd: "Hej ts updated4",
      build_cmd: "Hej ts updated3",
      branch: "Hej ts updated2",
      git_url: "Hej ts updated1",
    }
    return this.http.post(`http://localhost:4200/api/processes/update/${id}`, model, {headers: header, responseType: "text"});
  }
}
