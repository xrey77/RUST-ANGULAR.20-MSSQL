import { Injectable, inject } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import {  Observable } from 'rxjs';
import { Profile } from '../interface/profile';

interface ActivateMfa {
  twofactorenabled: boolean;
}

@Injectable({
  providedIn: 'root'
})

export class Profileservice {
  
  private http = inject(HttpClient)
    
  public getUserbyId(id: any, token: any): Observable<any> {
    const options = {
      headers: new HttpHeaders({
        'Content-Type':  'application/json',
        'Authorization': `Bearer ${token}`
      })
    };    
    return this.http.get(`http://127.0.0.1:3000/api/getuserid/${id}`, options);
  }

  public ActivateMFA(id: any, enabled: ActivateMfa, token: any) {
    const options = {
      headers: new HttpHeaders({
        'Content-Type':  'application/json',
        'Authorization': `Bearer ${token}`
      })
    };    
    return this.http.patch(`http://127.0.0.1:3000/api/mfa/activate/${id}`, enabled, options);
  }

  public UploadPicture(id: any, profilepic: any, token: any): Observable<any> {
    const options = {
      headers: new HttpHeaders({
        'Authorization': `Bearer ${token}`
      })
    }; 
    return this.http.patch<any>(`http://127.0.0.1:3000/api/uploadpicture/${id}`, profilepic, options);
  }

  public sendProfileRequest(id: any,userdtls: any, token: any): Observable<any> {
    const options = {
      headers: new HttpHeaders({
        'Content-Type':  'application/json',
        'Authorization': `Bearer ${token}`
      })
    };    
    return this.http.patch<Profile>(`http://127.0.0.1:3000/api/updateprofile/${id}`, userdtls, options);
  }  

  public sendNewpasswordRequest(id: any, userdtls: any, token: any): Observable<any> {
    const options = {
      headers: new HttpHeaders({
        'Content-Type':  'application/json',
        'Authorization': `Bearer ${token}`
      })
    };    
    return this.http.patch<Profile>(`http://127.0.0.1:3000/api/changepassword/${id}`, userdtls, options);
  }  
  
}
