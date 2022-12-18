import { HttpClient, HttpHeaderResponse, HttpHeaders } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { BehaviorSubject, Observable } from 'rxjs';
import { environment } from 'src/environments/environment';
import { Process } from '../models/Processes';

@Injectable({
  providedIn: 'root'
})
export class ApiProcessesService {

  constructor(private http: HttpClient) {
    this.header.set("Access-Control-Allow-Origin", "*")
    this.header.set("Access-Control-Allow-Methods", "DELETE, POST, GET, OPTIONS")
    this.header.set("Access-Control-Allow-Headers", "Content-Type, Authorization, X-Requested-With")
   }

  header = new HttpHeaders();
  url = environment.apiUrl;

  getProcesses() {
    let header: HttpHeaders = this.header;
    return this.http.get(this.url+"/api/processes/get_processes", {withCredentials: true, headers: header});
  }

  startProcess(id: number) {
    let header: HttpHeaders = this.header;
    console.log("start")
    return this.http.post(this.url+`/api/processes/start/${id}`, null, {withCredentials: true, headers: header});
  }

  stopProcess(id: number) {
    let header: HttpHeaders = this.header;
    return this.http.post(this.url+`/api/processes/stop/${id}`, null, {withCredentials: true, headers: header});
  }

  restartProcess(id: number) {
    let header: HttpHeaders = this.header;
    return this.http.post(this.url+`/api/processes/restart/${id}`, null, {withCredentials: true, headers: header});
  }

  create(name: string, start: string, stop: string, build: string, git: string, branch: string) {
    let header: HttpHeaders = this.header;
    let model = {
      name: name,
      start_cmd: start,
      stop_cmd: stop,
      build_cmd: build,
      branch: branch,
      git_url: git,
    }
    return this.http.post(this.url+`/api/processes/create`, model, {headers: header, responseType: "text", withCredentials: true});
  }

  update(id: number) {
    let header: HttpHeaders = this.header;
    let model = {
      name: "Hej ts updated7",
      path: "Hej ts updated6",
      start_cmd: "Hej ts updated5",
      stop_cmd: "Hej ts updated4",
      build_cmd: "Hej ts updated3",
      branch: "Hej ts updated2",
      git_url: "Hej ts updated1",
    }
    return this.http.post(this.url+`/api/processes/update/${id}`, model, {headers: header, responseType: "text", withCredentials: true});
  }
}
