# application中qa.rs点赞核心片段

```rust
// qa-svc/src/application/qa.rs 文件
// 回答点赞
async fn answer_agree(
    &self,
    request: Request<AnswerAgreeRequest>,
) -> Result<Response<AnswerAgreeReply>, Status> {
    let req = request.into_inner();
    info!("request username:{} answer_id:{}", req.created_by, req.id);
    let answer_res = self.answer_repo.fetch_one(req.id).await;
    if let Err(err) = answer_res {
        return Err(Status::new(
            Code::Unknown,
            format!("failed to query answer,error:{}", err),
        ));
    }

    // 判断是否点赞过
    let is_voted = self
        .vote_repo
        .is_voted(req.id, "answer", &req.created_by)
        .await
        .unwrap_or(false);
    if req.action == "up" {
        if is_voted {
            let reply = AnswerAgreeReply {
                state: 0,
                reason: "you already voted it".to_string(),
                agree_count: 0,
            };
            return Ok(Response::new(reply));
        }
    } else {
        if !is_voted {
            let reply = AnswerAgreeReply {
                state: 0,
                reason: "You didn't cancel vote,bad request".to_string(),
                agree_count: 0,
            };
            return Ok(Response::new(reply));
        }
    }

    // 是否要返回点赞数，可根据实际业务场景决定
    let mut agree_count = answer_res.unwrap().agree_count as i64;
    if req.action == "up" {
        agree_count += 1;
    } else {
        if agree_count >= 1 {
            agree_count -= 1;
        }
    }
    let msg = VoteMessage {
        target_id: req.id,
        target_type: "answer".to_string(),
        created_by: req.created_by,
        action: req.action,
    };
    let res = self.vote_repo.publish(msg).await;
    if let Err(err) = res {
        return Err(Status::new(
            Code::Unknown,
            format!("failed to vote answer,error:{}", err),
        ));
    }

    let reply = AnswerAgreeReply {
        state: 1,
        reason: "success".to_string(),
        agree_count: agree_count as u64,
    };

    Ok(Response::new(reply))
}
```

- 在上述answer_agree方法中，当接收到用户点赞或取消点赞的请求后，首先根据回答id查询当前回答基本信息，判断是否存在，如果不存在就返回对应的错误信息。
- 如果回答存在，就先调用vote_repo（UserVoteRepo trait实例）对象上面的is_voted方法判断当前用户是否点赞过。
- 如果req.action等于up，is_voted为true，就返回已点赞的错误提示。如果req.action不等于up，is_voted为false，表示用户未点赞，就返回非法请求的错误提示。
- 随后，根据req.action不同，将点赞数agree_count执行自增或自减操作，是否需要返回点赞数，可以根据具体业务场景决定。
- 最后，调用vote_repo.publish方法将点赞或取消点赞的消息msg发送到pulsar消息队列中，并通过if
  let模式匹配判断执行过程中是否发生错误。如果发生错误，就提示对应的错误信息，否则就返回操作成功所对应的AnswerAgreeReply实例对象。
