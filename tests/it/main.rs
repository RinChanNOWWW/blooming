// Copyright 2023 RinChanNOWWW
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

use std::fs::File;

use bloom::ByrbtRSSContent;
use bloom::Item;
use bloom::MikanRSSContent;
use bloom::Result;
use bloom::TjuptRSSContent;
use chrono::DateTime;
use chrono::Local;

#[test]
fn test_parse_mikan() -> Result<()> {
    let file = File::open("tests/it/testdata/mikan.xml")?;
    let content: MikanRSSContent = serde_xml_rs::from_reader(file)?;

    let items = content
        .channel
        .items
        .into_iter()
        .map(Item::from)
        .collect::<Vec<_>>();
    let expected =
        vec![Item {
        title: "【豌豆字幕组】[海盗战记 / 冰海战记 第二季 / Vinland_Saga_S2][03][简体][1080P][MP4]"
            .to_string(),
        pub_date: DateTime::parse_from_rfc3339("2023-01-24T14:34:31.721+08:00")
            .unwrap()
            .with_timezone(&Local {}),
    },Item {
        title: "[ANi] The Vampire Dies in No Time S2 - 吸血鬼马上死 第二季 - 03 [1080P][Baha][WEB-DL][AAC AVC][CHT][MP4]"
            .to_string(),
        pub_date: DateTime::parse_from_rfc3339("2023-01-23T21:37:12.436+08:00")
            .unwrap()
            .with_timezone(&Local {}),
    }];

    assert_eq!(items, expected);

    Ok(())
}

#[test]
fn test_parse_byrbt() -> Result<()> {
    let file = File::open("tests/it/testdata/byrbt.xml")?;
    let content: ByrbtRSSContent = serde_xml_rs::from_reader(file)?;

    let items = content
        .channel
        .items
        .into_iter()
        .map(Item::from)
        .collect::<Vec<_>>();
    let expected =
        vec![Item {
        title: "[大陆][三体][Three-Body.S01.2023.WEB-DL.4K.H265.AAC-venti][S01E12][MP4]"
            .to_string(),
        pub_date: DateTime::parse_from_rfc2822("Tue, 24 Jan 2023 21:29:39 +0800")
            .unwrap()
            .with_timezone(&Local {}),
    },Item {
        title: "[大陆][三体][Three.Body.S01.2023.2160p.DV.WEB-DL.H265.DDP5.1.Atmos-CHDWEB][E10-E11][MP4]"
            .to_string(),
        pub_date: DateTime::parse_from_rfc2822("Tue, 24 Jan 2023 20:21:43 +0800")
            .unwrap()
            .with_timezone(&Local {}),
    }];

    assert_eq!(items, expected);

    Ok(())
}

#[test]
fn test_parse_tjubt() -> Result<()> {
    let file = File::open("tests/it/testdata/tjupt.xml")?;
    let content: TjuptRSSContent = serde_xml_rs::from_reader(file)?;

    let items = content
        .channel
        .items
        .into_iter()
        .map(Item::from)
        .collect::<Vec<_>>();
    let expected =
        vec![Item {
        title: "[电影][意大利/法国][阿玛柯德][Amarcord.1973.Criterion.Collection.1080p.BluRay.x264-WiKi][阿玛柯德/我记得/想当年(港)/阿玛珂德(台)][14.10GiB]"
            .to_string(),
        pub_date: DateTime::parse_from_rfc2822("Mon, 03 May 2021 05:35:49 +0000")
            .unwrap()
            .with_timezone(&Local {}),
    }];

    assert_eq!(items, expected);

    Ok(())
}
