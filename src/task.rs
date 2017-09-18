//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

//! Module containing `Task` type as well as trait implementations

use uuid::Uuid;

use priority::TaskPriority;
use status::TaskStatus;
use project::Project;
use tag::Tag;
use date::Date;
use annotation::Annotation;

/// Task type
///
/// A task must have four things:
///
/// - A Status
/// - An UUID
/// - An Entry-Date
/// - A Description
///
/// all other Data is optional by taskwarrior. This type is a simple rust representation of the
/// JSON exported by taskwarrior.
///
/// It is deserializeable and serializeable via serde_json, so importing and exporting taskwarrior
/// tasks is simply serializing and deserializing objects of this type.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Task {
    status      : TaskStatus,
    uuid        : Uuid,
    entry       : Date,
    description : String,

    #[serde(skip_serializing_if = "Option::is_none")]
    annotations : Option<Vec<Annotation>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    depends     : Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    due         : Option<Date>,

    #[serde(skip_serializing_if = "Option::is_none")]
    end         : Option<Date>,

    #[serde(skip_serializing_if = "Option::is_none")]
    imask       : Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    mask        : Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    modified    : Option<Date>,

    #[serde(skip_serializing_if = "Option::is_none")]
    parent      : Option<Uuid>,

    #[serde(skip_serializing_if = "Option::is_none")]
    priority    : Option<TaskPriority>,

    #[serde(skip_serializing_if = "Option::is_none")]
    project     : Option<Project>,

    #[serde(skip_serializing_if = "Option::is_none")]
    recur       : Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    scheduled   : Option<Date>,

    #[serde(skip_serializing_if = "Option::is_none")]
    start       : Option<Date>,

    #[serde(skip_serializing_if = "Option::is_none")]
    tags        : Option<Vec<Tag>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    until       : Option<Date>,

    #[serde(skip_serializing_if = "Option::is_none")]
    wait        : Option<Date>,
}

/*
 * TODO: We do not fail if the JSON parsing fails. This panics. We rely on taskwarrior to be nice
 * to us. I guess this should be fixed.
 */
impl Task {

    /// Create a new Task instance
    pub fn new(
        status      : TaskStatus,
        uuid        : Uuid,
        entry       : Date,
        description : String,

        annotations : Option<Vec<Annotation>>,
        depends     : Option<String>,
        due         : Option<Date>,
        end         : Option<Date>,
        imask       : Option<i64>,
        mask        : Option<String>,
        modified    : Option<Date>,
        parent      : Option<Uuid>,
        priority    : Option<TaskPriority>,
        project     : Option<Project>,
        recur       : Option<String>,
        scheduled   : Option<Date>,
        start       : Option<Date>,
        tags        : Option<Vec<Tag>>,
        until       : Option<Date>,
        wait        : Option<Date>,
    ) -> Task
    {
        Task {
            status      : status,
            uuid        : uuid,
            entry       : entry,
            description : description,

            annotations : annotations,
            depends     : depends,
            due         : due,
            end         : end,
            imask       : imask,
            mask        : mask,
            modified    : modified,
            parent      : parent,
            priority    : priority,
            project     : project,
            recur       : recur,
            scheduled   : scheduled,
            start       : start,
            tags        : tags,
            until       : until,
            wait        : wait,
        }
    }

    /// Get the status of the task
    pub fn status(&self) -> &TaskStatus {
        &self.status
    }

    /// Get the status of the task mutable
    pub fn status_mut(&mut self) -> &mut TaskStatus {
        &mut self.status
    }

    /// Get the uuid of the task
    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    /// Get the uuid of the task mutable
    pub fn uuid_mut(&mut self) -> &mut Uuid {
        &mut self.uuid
    }

    /// Get the entry date of the task
    pub fn entry(&self) -> &Date {
        &self.entry
    }

    /// Get the entry date of the task mutable
    pub fn entry_mut(&mut self) -> &mut Date {
        &mut self.entry
    }

    /// Get the description of the task
    pub fn description(&self) -> &String {
        &self.description
    }

    /// Get the description of the task mutable
    pub fn description_mut(&mut self) -> &mut String {
        &mut self.description
    }

    /// Get the annotations of the task
    pub fn annotations(&self) -> Option<&Vec<Annotation>> {
        self.annotations.as_ref()
    }

    /// Get the annotations of the task mutable
    pub fn annotations_mut(&mut self) -> Option<&mut Vec<Annotation>> {
        self.annotations.as_mut()
    }

    /// Get the depends of the task
    ///
    /// This is exported as String by now, which might change in future
    pub fn depends(&self) -> Option<&String> {
        self.depends.as_ref()
    }

    /// This is exported as String by now, which might change in future mutable
    pub fn depends_mut(&mut self) -> Option<&mut String> {
        self.depends.as_mut()
    }

    /// Get the due date of the task
    pub fn due(&self) -> Option<&Date> {
        self.due.as_ref()
    }

    /// Get the due date of the task mutable
    pub fn due_mut(&mut self) -> Option<&mut Date> {
        self.due.as_mut()
    }

    /// Get the end date of the task
    pub fn end(&self) -> Option<&Date> {
        self.end.as_ref()
    }

    /// Get the end date of the task mutable
    pub fn end_mut(&mut self) -> Option<&mut Date> {
        self.end.as_mut()
    }

    /// Get the imask of the task
    pub fn imask(&self) -> Option<&i64> {
        self.imask.as_ref()
    }

    /// Get the imask of the task mutable
    pub fn imask_mut(&mut self) -> Option<&mut i64> {
        self.imask.as_mut()
    }

    /// Get the mask of the task
    pub fn mask(&self) -> Option<&String> {
        self.mask.as_ref()
    }

    /// Get the mask of the task mutable
    pub fn mask_mut(&mut self) -> Option<&mut String> {
        self.mask.as_mut()
    }

    /// Get the modified date of the task
    pub fn modified(&self) -> Option<&Date> {
        self.modified.as_ref()
    }

    /// Get the modified date of the task mutable
    pub fn modified_mut(&mut self) -> Option<&mut Date> {
        self.modified.as_mut()
    }

    /// Get the parent of the task
    pub fn parent(&self) -> Option<&Uuid> {
        self.parent.as_ref()
    }

    /// Get the parent of the task mutable
    pub fn parent_mut(&mut self) -> Option<&mut Uuid> {
        self.parent.as_mut()
    }

    /// Get the priority of the task
    pub fn priority(&self) -> Option<&TaskPriority> {
        self.priority.as_ref()
    }

    /// Get the priority of the task mutable
    pub fn priority_mut(&mut self) -> Option<&mut TaskPriority> {
        self.priority.as_mut()
    }

    /// Get the project of the task
    pub fn project(&self) -> Option<&Project> {
        self.project.as_ref()
    }

    /// Get the project of the task mutable
    pub fn project_mut(&mut self) -> Option<&mut Project> {
        self.project.as_mut()
    }

    /// Get the recur of the task
    ///
    /// This is exported as String by now. This might change in future versions of this crate.
    pub fn recur(&self) -> Option<&String> {
        self.recur.as_ref()
    }

    /// This is exported as String by now. This might change in future versions of this crate.
    /// mutable
    pub fn recur_mut(&mut self) -> Option<&mut String> {
        self.recur.as_mut()
    }

    /// Get the scheduled date of the task
    pub fn scheduled(&self) -> Option<&Date> {
        self.scheduled.as_ref()
    }

    /// Get the scheduled date of the task mutable
    pub fn scheduled_mut(&mut self) -> Option<&mut Date> {
        self.scheduled.as_mut()
    }

    /// Get the start date of the task
    pub fn start(&self) -> Option<&Date> {
        self.start.as_ref()
    }

    /// Get the start date of the task mutable
    pub fn start_mut(&mut self) -> Option<&mut Date> {
        self.start.as_mut()
    }

    /// Get the tags of the task
    pub fn tags(&self) -> Option<&Vec<Tag>> {
        self.tags.as_ref()
    }

    /// Get the tags of the task mutable
    pub fn tags_mut(&mut self) -> Option<&mut Vec<Tag>> {
        self.tags.as_mut()
    }

    /// Get the until date of the task
    pub fn until(&self) -> Option<&Date> {
        self.until.as_ref()
    }

    /// Get the until date of the task mutable
    pub fn until_mut(&mut self) -> Option<&mut Date> {
        self.until.as_mut()
    }

    /// Get the wait date of the task
    pub fn wait(&self) -> Option<&Date> {
        self.wait.as_ref()
    }

    /// Get the wait date of the task mutable
    pub fn wait_mut(&mut self) -> Option<&mut Date> {
        self.wait.as_mut()
    }

}

#[cfg(test)]
mod test {
    use date::Date;
    use date::TASKWARRIOR_DATETIME_TEMPLATE;
    use status::TaskStatus;
    use task::Task;
    use annotation::Annotation;

    use uuid::Uuid;
    use chrono::NaiveDateTime;
    use serde_json;

    fn mkdate(s: &str) -> Date {
        let n = NaiveDateTime::parse_from_str(s, TASKWARRIOR_DATETIME_TEMPLATE);
        Date::from(n.unwrap())
    }

    #[test]
    fn test_deser() {
        let s =
r#"{
"id": 1,
"description": "test",
"entry": "20150619T165438Z",
"status": "waiting",
"uuid": "8ca953d5-18b4-4eb9-bd56-18f2e5b752f0"
}"#;

        println!("{}", s);

        let task = serde_json::from_str(s);
        println!("{:?}", task);
        assert!(task.is_ok());
        let task : Task = task.unwrap();

        assert!(task.status().clone() == TaskStatus::Waiting);
        assert!(task.description() == "test");
        assert!(task.entry().clone() == mkdate("20150619T165438Z"));
        assert!(task.uuid().clone() == Uuid::parse_str("8ca953d5-18b4-4eb9-bd56-18f2e5b752f0").unwrap());

        let back = serde_json::to_string(&task).unwrap();

        assert!(back.contains("description"));
        assert!(back.contains("test"));
        assert!(back.contains("entry"));
        assert!(back.contains("20150619T165438Z"));
        assert!(back.contains("status"));
        assert!(back.contains("waiting"));
        assert!(back.contains("uuid"));
        assert!(back.contains("8ca953d5-18b4-4eb9-bd56-18f2e5b752f0"));
    }

    #[test]
    fn test_deser_more() {
        let s =
r#"{
"id": 1,
"description": "some description",
"entry": "20150619T165438Z",
"modified": "20160327T164007Z",
"project": "someproject",
"status": "waiting",
"tags": ["some", "tags", "are", "here"],
"uuid": "8ca953d5-18b4-4eb9-bd56-18f2e5b752f0",
"wait": "20160508T164007Z",
"urgency": 0.583562
}"#;

        println!("{}", s);

        let task = serde_json::from_str(s);
        println!("{:?}", task);
        assert!(task.is_ok());
        let task : Task = task.unwrap();

        assert!(task.status().clone() == TaskStatus::Waiting);
        assert!(task.description() == "some description");
        assert!(task.entry().clone() == mkdate("20150619T165438Z"));
        assert!(task.uuid().clone() == Uuid::parse_str("8ca953d5-18b4-4eb9-bd56-18f2e5b752f0").unwrap());

        assert!(task.modified() == Some(&mkdate("20160327T164007Z")));
        assert!(task.project() == Some(&String::from("someproject")));

        if let Some(tags) = task.tags() {
            for tag in tags {
                let any_tag = [ "some", "tags", "are", "here", ]
                    .into_iter().any(|t| tag == *t);
                assert!(any_tag, "Tag {} missing", tag);
            }
        } else {
                assert!(false, "Tags completely missing");
        }

        assert!(task.wait() == Some(&mkdate("20160508T164007Z")));
        // assert!(task.urgency().clone() == 0.583562);

        let back = serde_json::to_string(&task).unwrap();

        assert!(back.contains("description"));
        assert!(back.contains("some description"));
        assert!(back.contains("entry"));
        assert!(back.contains("20150619T165438Z"));
        assert!(back.contains("project"));
        assert!(back.contains("someproject"));
        assert!(back.contains("status"));
        assert!(back.contains("waiting"));
        assert!(back.contains("tags"));
        assert!(back.contains("some"));
        assert!(back.contains("tags"));
        assert!(back.contains("are"));
        assert!(back.contains("here"));
        assert!(back.contains("uuid"));
        assert!(back.contains("8ca953d5-18b4-4eb9-bd56-18f2e5b752f0"));
    }

    #[test]
    fn test_deser_annotation() {
        let s =
r#"{
"id":192,
"description":"Some long description for a task",
"entry":"20160423T125820Z",
"modified":"20160423T125942Z",
"project":"project",
"status":"pending",
"tags":["search","things"],
"uuid":"5a04bb1e-3f4b-49fb-b9ba-44407ca223b5",
"annotations":[{"entry":"20160423T125911Z","description":"An Annotation"},
               {"entry":"20160423T125926Z","description":"Another Annotation"},
               {"entry":"20160422T125942Z","description":"A Third Anno"}
               ],
"urgency":10.911
}"#;

        println!("{}", s);

        let task = serde_json::from_str(s);
        println!("{:?}", task);
        assert!(task.is_ok());
        let task : Task = task.unwrap();

        let all_annotations = vec![
            Annotation::new(mkdate("20160423T125911Z"), String::from("An Annotation")),
            Annotation::new(mkdate("20160423T125926Z"), String::from("Another Annotation")),
            Annotation::new(mkdate("20160422T125942Z"), String::from("A Third Anno"))
        ];

        if let Some(annotations) = task.annotations() {
            for annotation in annotations {
                let r = all_annotations.iter().any(|ann| {
                    let descr = ann.description() == annotation.description();
                    let entry = ann.entry() == annotation.entry();

                    descr && entry
                });
                assert!(r, "Annotation {:?} missing or buggy", annotation);
            }
        } else {
            assert!(false, "Annotations missing");
        }
    }

}

