// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

import { Component, Input, OnInit } from "@angular/core";
import { FormControl, FormGroup, FormBuilder, Validators } from "@angular/forms";
import { List } from "immutable";
import config from "../../../config";
import { AppStore } from "../../../AppStore";
import { fetchOriginInvitations } from "../../origin.actions";
import { fetchOriginMembers, inviteUserToOrigin } from "./origin-members-tab.actions";
import { ActivatedRoute } from "@angular/router";
import { Subscription } from "rxjs/Subscription";
import { OriginRecord } from "../../../records/origin-record";

@Component({
    selector: "hab-origin-members-tab",
    template: require("./origin-members-tab.component.html")
})

export class OriginMembersTabComponent implements OnInit {
    form: FormGroup;
    control: FormControl;

    constructor(private route: ActivatedRoute, formBuilder: FormBuilder, private store: AppStore) {
        this.form = formBuilder.group({});
    }

    submit(username: string) {
        this.onSubmit(username);
    }

    public ngOnInit() {
        this.store.dispatch(fetchOriginMembers(
            this.origin.name, this.gitHubAuthToken
        ));
        this.store.dispatch(fetchOriginInvitations(
            this.origin.name, this.gitHubAuthToken
        ));
        this.control = new FormControl("", Validators.required);
        this.form.addControl("username", this.control);
    }

    get origin() {
        return this.store.getState().origin.current;
    }

    get ui() {
        return this.store.getState().origins.ui.current;
    }

    get errorMessage() {
        return this.ui.userInviteErrorMessage;
    }

    get invitations(): List<Object> {
        return this.store.getState().origins.currentPendingInvitations;
    }

    get members(): List<Object> {
        return this.store.getState().origins.currentMembers;
    }

    get docsUrl() {
        return config["docs_url"];
    }

    get gitHubAuthToken() {
        return this.store.getState().gitHub.authToken;
    }

    onSubmit(username) {
        this.store.dispatch(inviteUserToOrigin(
            username,
            this.origin.name,
            this.gitHubAuthToken
        ));
    }
}