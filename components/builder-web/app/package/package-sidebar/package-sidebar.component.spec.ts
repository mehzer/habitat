import { TestBed, ComponentFixture } from "@angular/core/testing";
import { Component, DebugElement, SimpleChange } from "@angular/core";
import { By } from "@angular/platform-browser";
import { RouterTestingModule } from "@angular/router/testing";
import { List } from "immutable";
import { MockComponent } from "ng2-mock-component";
import { AppStore } from "../../AppStore";
import * as actions from "../../actions/index";
import { Package } from "../../records/Package";
import { PackageSidebarComponent } from "./package-sidebar.component";

class MockAppStore {
  static state;

  getState() {
    return MockAppStore.state;
  }

  dispatch() {}
}

describe("PackageSidebarComponent", () => {
  let fixture: ComponentFixture<PackageSidebarComponent>;
  let component: PackageSidebarComponent;
  let element: DebugElement;
  let store: MockAppStore;

  beforeEach(() => {

    TestBed.configureTestingModule({
      imports: [
        RouterTestingModule
      ],
      declarations: [
        PackageSidebarComponent,
        MockComponent({ selector: "hab-copyable", inputs: [ "command" ] }),
        MockComponent({ selector: "hab-platform-icon", inputs: [ "platform" ]})
      ],
      providers: [
        { provide: AppStore, useClass: MockAppStore }
      ]
    });

    fixture = TestBed.createComponent(PackageSidebarComponent);
    component = fixture.componentInstance;
    element = fixture.debugElement;
    store = TestBed.get(AppStore);
  });

  beforeEach(() => {
    MockAppStore.state = {
      packages: {
        latestInChannel: {
          stable: {
            ident: {
              origin: "core",
              name: "nginx",
              version: "1.11.10"
            }
          }
        }
      },
      session: {
        token: undefined
      },
      gitHub: {
        authToken: undefined
      },
      origins: {
        mine: List()
      }
    };
  });

  describe("given an origin and name", () => {

    beforeEach(() => {
      spyOn(store, "dispatch");
      spyOn(actions, "fetchLatestInChannel");

      component.ngOnChanges({
        origin: new SimpleChange(undefined, "core", true),
        name: new SimpleChange(undefined, "nginx", true)
      });
    });

    it("fetches the latest package", () => {
      expect(store.dispatch).toHaveBeenCalled();
      expect(actions.fetchLatestInChannel).toHaveBeenCalledWith("core", "nginx", "stable");
    });

    it("hides the build button", () => {
      expect(element.query(By.css(".hab-package-sidebar .build button"))).toBeNull();
    });

    describe("when buildable", () => {

      beforeEach(() => {
        component.buildable = true;
      });

      it("shows the build button", () => {
        fixture.detectChanges();
        expect(element.query(By.css(".hab-package-sidebar .build button"))).not.toBeNull();
      });

      describe("and building", () => {

        beforeEach(() => {
          component.building = true;
        });

        it("disables the build button", () => {
          fixture.detectChanges();

          let el = element.query(By.css(".hab-package-sidebar .build button")).nativeElement;
          expect(el.getAttribute("disabled")).not.toBeNull();
        });
      });
    });
  });
});
