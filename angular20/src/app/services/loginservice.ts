import { Injectable, inject } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { User } from '../interface/user';
import {  Observable } from 'rxjs';

@Injectable({
  providedIn: 'root'
})

export class Loginservice {
 
    private http = inject(HttpClient);
    private apiUrl = "http://127.0.0.1:3000/auth/signin";

  public sendLoginRequest(userdtls: any): Observable<any> {
    const options = {
      headers: new HttpHeaders({
        'Content-Type':  'application/json',
      })
    };
      return this.http.post<User>(this.apiUrl ,userdtls, options);
  };
 


}
