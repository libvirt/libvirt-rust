/*
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library.  If not, see
 * <https://www.gnu.org/licenses/>.
 *
 * Sahid Orentino Ferdjaoui <sahid.ferdjaoui@redhat.com>
 */

mod common;

use uuid::Uuid;

use virt::domain::{Domain, MemoryParameters, NUMAParameters, SchedulerInfo};
use virt::error::ErrorNumber;
use virt::sys;

fn tdom(exec_test: fn(dom: Domain)) {
    let c = common::conn();
    match Domain::lookup_by_name(&c, "test") {
        Ok(dom) => {
            exec_test(dom);
            common::close(c);
        }
        Err(e) => panic!("{e}"),
    }
}

#[test]
fn test_name() {
    fn t(dom: Domain) {
        assert_eq!("test", dom.get_name().unwrap_or_default());
    }
    tdom(t);
}

#[test]
fn test_uuid_string() {
    fn t(dom: Domain) {
        assert_eq!(
            "6695eb01-f6a4-8304-79aa-97f2502e193f",
            dom.get_uuid_string().unwrap_or_default()
        );
    }
    tdom(t);
}

#[test]
fn test_uuid() {
    let uuid = Uuid::parse_str("6695eb01-f6a4-8304-79aa-97f2502e193f").unwrap_or_default();
    let c = common::conn();
    match Domain::lookup_by_uuid(&c, uuid) {
        Ok(dom) => {
            assert_eq!(uuid, dom.get_uuid().unwrap_or_default());
        }
        Err(e) => panic!("{e}"),
    };
    common::close(c);
}

#[test]
fn test_id() {
    fn t(dom: Domain) {
        assert_eq!(1, dom.get_id().unwrap_or(0));
    }
    tdom(t);
}

#[test]
fn test_get_xml_desc() {
    fn t(dom: Domain) {
        assert!(
            "" != dom.get_xml_desc(0).unwrap_or_default(),
            "Should not be empty"
        );
    }
    tdom(t);
}

#[test]
fn test_get_info() {
    fn t(dom: Domain) {
        match dom.get_info() {
            Ok(info) => assert_eq!(1, info.state),
            Err(_) => panic!("should have a node info"),
        }
    }
    tdom(t);
}

#[test]
fn test_get_vcpus_flags() {
    fn t(dom: Domain) {
        assert_eq!(2, dom.get_vcpus_flags(0).unwrap_or(0));
    }
    tdom(t);
}

#[test]
fn test_schedinfo() {
    fn t(dom: Domain) {
        let info = dom.get_scheduler_parameters().unwrap();
        assert_eq!(info.scheduler_type, "fair");
        assert_eq!(info.weight, Some(50));
        assert_eq!(info.shares, None);
        assert_eq!(info.reservation, None);
        assert_eq!(info.limit, None);
        assert_eq!(info.cap, None);

        let newinfo = SchedulerInfo {
            weight: Some(37),
            ..Default::default()
        };
        dom.set_scheduler_parameters(&newinfo).unwrap();

        /*
         * XXX test:///default driver doesn't currently persist the
         * changes made with 'set', so we can't test roundtrip
         *
         *  let newerinfo = dom.get_scheduler_parameters().unwrap();
         *
         *  assert_eq!(newerinfo.weight, Some(37));
         */
    }
    tdom(t);
}

#[test]
fn test_memory_params() {
    fn t(dom: Domain) {
        let info = dom.get_memory_parameters(0).unwrap();
        assert_eq!(info.hard_limit, Some(MemoryParameters::VALUE_UNLIMITED));
        assert_eq!(info.soft_limit, Some(MemoryParameters::VALUE_UNLIMITED));
        assert_eq!(
            info.swap_hard_limit,
            Some(MemoryParameters::VALUE_UNLIMITED)
        );
        assert_eq!(info.min_guarantee, None);

        let newinfo = MemoryParameters {
            soft_limit: Some(87539319),
            ..Default::default()
        };
        dom.set_memory_parameters(newinfo, 0).unwrap();

        let info = dom.get_memory_parameters(0).unwrap();
        assert_eq!(info.hard_limit, Some(MemoryParameters::VALUE_UNLIMITED));
        assert_eq!(info.soft_limit, Some(87539319));
        assert_eq!(
            info.swap_hard_limit,
            Some(MemoryParameters::VALUE_UNLIMITED)
        );
        assert_eq!(info.min_guarantee, None);
    }
    tdom(t);
}

#[test]
fn test_numa_params() {
    fn t(dom: Domain) {
        let info = dom.get_numa_parameters(0).unwrap();
        assert_eq!(info.mode, Some(sys::VIR_DOMAIN_NUMATUNE_MEM_STRICT as i32));
        assert_eq!(info.node_set, Some("".to_string()));

        let newinfo = NUMAParameters {
            node_set: Some("1,2".to_string()),
            mode: Some(sys::VIR_DOMAIN_NUMATUNE_MEM_PREFERRED as i32),
        };
        dom.set_numa_parameters(newinfo, 0).unwrap();

        let newerinfo = dom.get_numa_parameters(0).unwrap();
        assert_eq!(
            newerinfo.mode,
            Some(sys::VIR_DOMAIN_NUMATUNE_MEM_PREFERRED as i32)
        );
        // Libvirt canonicalizes the pair of nodes to a range
        assert_eq!(newerinfo.node_set, Some("1-2".to_string()));
    }
    tdom(t);
}

#[test]
fn test_lookup_domain_by_id() {
    let c = common::conn();
    let d = common::build_test_domain(&c, "by_id", true);
    let id = d.get_id().unwrap_or(0);
    match Domain::lookup_by_id(&c, id) {
        Ok(mut r) => r.free().unwrap_or(()),
        Err(e) => panic!("{e}"),
    }
    common::clean(d);
    common::close(c);
}

#[test]
fn test_lookup_domain_by_name() {
    let c = common::conn();
    match Domain::lookup_by_name(&c, "test") {
        Ok(mut r) => r.free().unwrap_or(()),
        Err(e) => panic!("{e}"),
    }
    common::close(c);
}

#[test]
fn test_create_with_flags() {
    let c = common::conn();
    let d = common::build_test_domain(&c, "create", false);
    assert_eq!(Ok(0), d.create_with_flags(0));
    assert_eq!(Ok((sys::VIR_DOMAIN_RUNNING, 1)), d.get_state());
    assert_eq!(Ok(String::from("libvirt-rs-test-create")), d.get_name());
    common::clean(d);
    common::close(c);
}

#[test]
fn test_shutdown() {
    let c = common::conn();
    let d = common::build_test_domain(&c, "shutdown", false);
    assert_eq!(Ok(0), d.create_with_flags(0));
    assert_eq!(Ok((sys::VIR_DOMAIN_RUNNING, 1)), d.get_state());
    assert_eq!(Ok(0), d.shutdown());
    assert_eq!(Ok((sys::VIR_DOMAIN_SHUTOFF, 1)), d.get_state());
    common::clean(d);
    common::close(c);
}

#[test]
fn test_pause_resume() {
    let c = common::conn();
    let d = common::build_test_domain(&c, "pause_resume", false);
    assert_eq!(Ok(0), d.create_with_flags(0));
    assert_eq!(Ok((sys::VIR_DOMAIN_RUNNING, 1)), d.get_state());
    assert_eq!(Ok(0), d.suspend());
    assert_eq!(Ok((sys::VIR_DOMAIN_PAUSED, 1)), d.get_state());
    assert_eq!(Ok(0), d.resume());
    assert_eq!(Ok((sys::VIR_DOMAIN_RUNNING, 5)), d.get_state());
    common::clean(d);
    common::close(c);
}

#[test]
fn test_screenshot() {
    let c = common::conn();
    let d = common::build_test_domain(&c, "screenshot", false);
    assert_eq!(Ok(0), d.create_with_flags(0));
    assert_eq!(Ok((sys::VIR_DOMAIN_RUNNING, 1)), d.get_state());

    let s = virt::stream::Stream::new(&c, 0).unwrap();
    assert_eq!(Ok(String::from("image/png")), d.screenshot(&s, 0, 0));
    assert_eq!(Ok(()), s.finish());

    common::clean(d);
    common::close(c);
}

#[test]
fn test_metadata() {
    let c = common::conn();
    let d = common::build_test_domain(&c, "metadata", false);

    assert_eq!(
        ErrorNumber::NoDomainMetadata,
        d.get_metadata(sys::VIR_DOMAIN_METADATA_DESCRIPTION as i32, None, 0)
            .unwrap_err()
            .code()
    );

    assert_eq!(
        Ok(0),
        d.set_metadata(
            sys::VIR_DOMAIN_METADATA_DESCRIPTION as i32,
            Some("fish"),
            None,
            None,
            0
        )
    );
    assert_eq!(
        Ok("fish".to_string()),
        d.get_metadata(sys::VIR_DOMAIN_METADATA_DESCRIPTION as i32, None, 0)
    );
    assert_eq!(
        Ok(0),
        d.set_metadata(
            sys::VIR_DOMAIN_METADATA_DESCRIPTION as i32,
            None,
            None,
            None,
            0
        )
    );

    assert_eq!(
        ErrorNumber::NoDomainMetadata,
        d.get_metadata(sys::VIR_DOMAIN_METADATA_DESCRIPTION as i32, None, 0)
            .unwrap_err()
            .code()
    );

    assert_eq!(
        ErrorNumber::NoDomainMetadata,
        d.get_metadata(sys::VIR_DOMAIN_METADATA_TITLE as i32, None, 0)
            .unwrap_err()
            .code()
    );

    assert_eq!(
        Ok(0),
        d.set_metadata(
            sys::VIR_DOMAIN_METADATA_TITLE as i32,
            Some("food"),
            None,
            None,
            0
        )
    );
    assert_eq!(
        Ok("food".to_string()),
        d.get_metadata(sys::VIR_DOMAIN_METADATA_TITLE as i32, None, 0)
    );
    assert_eq!(
        Ok(0),
        d.set_metadata(sys::VIR_DOMAIN_METADATA_TITLE as i32, None, None, None, 0)
    );

    assert_eq!(
        ErrorNumber::NoDomainMetadata,
        d.get_metadata(sys::VIR_DOMAIN_METADATA_TITLE as i32, None, 0)
            .unwrap_err()
            .code()
    );

    let xmldoc = "<location>\n  <planet>mars</planet>\n</location>";
    let xmlkey = "space";
    let xmlns = "https://libvirt.org/schemas/rust/test/space/1.0";

    assert_eq!(
        ErrorNumber::NoDomainMetadata,
        d.get_metadata(sys::VIR_DOMAIN_METADATA_ELEMENT as i32, Some(xmlns), 0)
            .unwrap_err()
            .code()
    );

    assert_eq!(
        Ok(0),
        d.set_metadata(
            sys::VIR_DOMAIN_METADATA_ELEMENT as i32,
            Some(xmldoc),
            Some(xmlkey),
            Some(xmlns),
            0
        )
    );
    assert_eq!(
        Ok(xmldoc.to_string()),
        d.get_metadata(sys::VIR_DOMAIN_METADATA_ELEMENT as i32, Some(xmlns), 0)
    );
    assert_eq!(
        Ok(0),
        d.set_metadata(
            sys::VIR_DOMAIN_METADATA_ELEMENT as i32,
            None,
            Some(xmlkey),
            Some(xmlns),
            0
        )
    );

    assert_eq!(
        ErrorNumber::NoDomainMetadata,
        d.get_metadata(sys::VIR_DOMAIN_METADATA_ELEMENT as i32, Some(xmlns), 0)
            .unwrap_err()
            .code()
    );

    common::clean(d);
    common::close(c);
}
